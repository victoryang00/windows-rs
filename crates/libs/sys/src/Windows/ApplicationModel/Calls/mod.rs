#[cfg(feature = "ApplicationModel_Calls_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls_Provider")]
pub mod Provider;
pub type CallAnswerEventArgs = *mut ::core::ffi::c_void;
pub type CallRejectEventArgs = *mut ::core::ffi::c_void;
pub type CallStateChangeEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct CellularDtmfMode(pub i32);
impl CellularDtmfMode {
    pub const Continuous: Self = Self(0i32);
    pub const Burst: Self = Self(1i32);
}
impl ::core::marker::Copy for CellularDtmfMode {}
impl ::core::clone::Clone for CellularDtmfMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct DtmfKey(pub i32);
impl DtmfKey {
    pub const D0: Self = Self(0i32);
    pub const D1: Self = Self(1i32);
    pub const D2: Self = Self(2i32);
    pub const D3: Self = Self(3i32);
    pub const D4: Self = Self(4i32);
    pub const D5: Self = Self(5i32);
    pub const D6: Self = Self(6i32);
    pub const D7: Self = Self(7i32);
    pub const D8: Self = Self(8i32);
    pub const D9: Self = Self(9i32);
    pub const Star: Self = Self(10i32);
    pub const Pound: Self = Self(11i32);
}
impl ::core::marker::Copy for DtmfKey {}
impl ::core::clone::Clone for DtmfKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct DtmfToneAudioPlayback(pub i32);
impl DtmfToneAudioPlayback {
    pub const Play: Self = Self(0i32);
    pub const DoNotPlay: Self = Self(1i32);
}
impl ::core::marker::Copy for DtmfToneAudioPlayback {}
impl ::core::clone::Clone for DtmfToneAudioPlayback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ICallAnswerEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceptedMedia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoipPhoneCallMedia) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallAnswerEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4252538391, data2: 11735, data3: 19596, data4: [178, 189, 149, 209, 122, 91, 183, 51] };
}
#[repr(C)]
pub struct ICallRejectEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RejectReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoipPhoneCallRejectReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallRejectEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3662150359, data2: 5076, data3: 19858, data4: [161, 194, 183, 120, 17, 238, 55, 236] };
}
#[repr(C)]
pub struct ICallStateChangeEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoipPhoneCallState) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICallStateChangeEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3937547422, data2: 26357, data3: 18425, data4: [159, 181, 69, 156, 81, 152, 199, 32] };
}
#[repr(C)]
pub struct ILockScreenCallEndCallDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenCallEndCallDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 769125645, data2: 39149, data3: 16449, data4: [150, 50, 80, 255, 129, 43, 119, 63] };
}
#[repr(C)]
pub struct ILockScreenCallEndRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
}
impl ::windows_sys::core::Interface for ILockScreenCallEndRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2173739875, data2: 28455, data3: 18153, data4: [174, 182, 192, 174, 131, 228, 125, 199] };
}
#[repr(C)]
pub struct ILockScreenCallUI {
    pub base__: ::windows_sys::core::IInspectable,
    pub Dismiss: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub CallTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCallTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILockScreenCallUI {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3315006861, data2: 29641, data3: 18964, data4: [176, 33, 236, 28, 80, 163, 183, 39] };
}
#[repr(C)]
pub struct IMuteChangeEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Muted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMuteChangeEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2240143705, data2: 3137, data3: 17196, data4: [129, 77, 197, 241, 253, 245, 48, 190] };
}
#[repr(C)]
pub struct IPhoneCall {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AudioDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioDeviceChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioDeviceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsMutedChanged: usize,
    pub CallId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallStatus) -> ::windows_sys::core::HRESULT,
    pub AudioDevice: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallAudioDevice) -> ::windows_sys::core::HRESULT,
    pub GetPhoneCallInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPhoneCallInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPhoneCallInfoAsync: usize,
    pub End: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EndAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndAsync: usize,
    pub SendDtmfKey: unsafe extern "system" fn(this: *mut *mut Self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendDtmfKeyAsync: unsafe extern "system" fn(this: *mut *mut Self, key: DtmfKey, dtmftoneaudioplayback: DtmfToneAudioPlayback, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendDtmfKeyAsync: usize,
    pub AcceptIncoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AcceptIncomingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptIncomingAsync: usize,
    pub Hold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HoldAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldAsync: usize,
    pub ResumeFromHold: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeFromHoldAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeFromHoldAsync: usize,
    pub Mute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MuteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteAsync: usize,
    pub Unmute: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnmuteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnmuteAsync: usize,
    pub RejectIncoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RejectIncomingAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectIncomingAsync: usize,
    pub ChangeAudioDevice: unsafe extern "system" fn(this: *mut *mut Self, endpoint: PhoneCallAudioDevice, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ChangeAudioDeviceAsync: unsafe extern "system" fn(this: *mut *mut Self, endpoint: PhoneCallAudioDevice, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAudioDeviceAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCall {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3243167992, data2: 49533, data3: 22994, data4: [150, 40, 102, 229, 69, 182, 205, 33] };
}
#[repr(C)]
pub struct IPhoneCallBlockingStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub BlockUnknownNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBlockUnknownNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BlockPrivateNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBlockPrivateNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetCallBlockingListAsync: unsafe extern "system" fn(this: *mut *mut Self, phonenumberlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetCallBlockingListAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallBlockingStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 426012548, data2: 11129, data3: 9969, data4: [164, 111, 105, 75, 224, 67, 243, 19] };
}
#[repr(C)]
pub struct IPhoneCallHistoryEntry {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsCallerIdBlocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCallerIdBlocked: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEmergency: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEmergency: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsMissed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsMissed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRinging: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRinging: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSuppressed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsVoicemail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVoicemail: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Media: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallHistoryEntryMedia) -> ::windows_sys::core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallHistoryEntryMedia) -> ::windows_sys::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallHistoryEntryOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSourceId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceIdKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallHistorySourceIdKind) -> ::windows_sys::core::HRESULT,
    pub SetSourceIdKind: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallHistorySourceIdKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4205895977, data2: 12964, data3: 19333, data4: [131, 209, 249, 13, 140, 35, 168, 87] };
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RawAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRawAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RawAddressKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallHistoryEntryRawAddressKind) -> ::windows_sys::core::HRESULT,
    pub SetRawAddressKind: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallHistoryEntryRawAddressKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryEntryAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 821123546, data2: 14677, data3: 16450, data4: [132, 230, 102, 238, 191, 130, 230, 127] };
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryAddressFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, rawaddress: ::windows_sys::core::HSTRING, rawaddresskind: PhoneCallHistoryEntryRawAddressKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryEntryAddressFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4212108730, data2: 51184, data3: 19382, data4: [159, 107, 186, 93, 115, 32, 154, 202] };
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub DesiredMedia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows_sys::core::HRESULT,
    pub SetDesiredMedia: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallHistoryEntryQueryDesiredMedia) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SourceIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SourceIds: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryEntryQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2623529308, data2: 35821, data3: 16586, data4: [176, 110, 196, 202, 142, 174, 92, 135] };
}
#[repr(C)]
pub struct IPhoneCallHistoryEntryReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryEntryReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1642915006, data2: 36230, data3: 18335, data4: [132, 4, 169, 132, 105, 32, 254, 230] };
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3643131171, data2: 62815, data3: 17235, data4: [157, 180, 2, 5, 165, 38, 90, 85] };
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: PhoneCallHistoryStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4121352761, data2: 45855, data3: 20293, data4: [172, 142, 27, 8, 137, 60, 27, 80] };
}
#[repr(C)]
pub struct IPhoneCallHistoryManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4023678192, data2: 41691, data3: 16776, data4: [158, 146, 188, 60, 250, 104, 19, 207] };
}
#[repr(C)]
pub struct IPhoneCallHistoryStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentryid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEntryAsync: usize,
    pub GetEntryReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetEntryReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, queryoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveEntryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteEntryAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteEntryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteEntriesAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteEntriesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkEntryAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkEntryAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkEntriesAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, callhistoryentries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkEntriesAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAllAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSourcesUnseenCountAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSourcesUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkSourcesAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkSourcesAsSeenAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallHistoryStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 797998520, data2: 46094, data3: 16939, data4: [133, 69, 203, 25, 16, 166, 28, 82] };
}
#[repr(C)]
pub struct IPhoneCallInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub IsHoldSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallDirection) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 582231415, data2: 15949, data3: 24006, data4: [137, 194, 70, 159, 229, 255, 193, 137] };
}
#[repr(C)]
pub struct IPhoneCallManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowPhoneCallUI: unsafe extern "system" fn(this: *mut *mut Self, phonenumber: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1626188920, data2: 30886, data3: 18546, data4: [163, 239, 152, 50, 94, 200, 184, 67] };
}
#[repr(C)]
pub struct IPhoneCallManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CallStateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CallStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCallStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCallStateChanged: usize,
    pub IsCallActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsCallIncoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShowPhoneCallSettingsUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3353594044, data2: 9072, data3: 17180, data4: [152, 253, 67, 190, 95, 3, 8, 109] };
}
#[repr(C)]
pub struct IPhoneCallStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFromId: unsafe extern "system" fn(this: *mut *mut Self, callid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 572059307, data2: 62987, data3: 21479, data4: [186, 19, 90, 234, 251, 194, 41, 87] };
}
#[repr(C)]
pub struct IPhoneCallStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsEmergencyPhoneNumberAsync: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEmergencyPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultLineAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultLineAsync: usize,
    pub RequestLineWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1600194376, data2: 6310, data3: 16755, data4: [134, 209, 40, 190, 157, 198, 45, 186] };
}
#[repr(C)]
pub struct IPhoneCallVideoCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsVideoCallingCapable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneCallVideoCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 37234566, data2: 45418, data3: 20443, data4: [190, 59, 196, 36, 14, 19, 173, 13] };
}
#[repr(C)]
pub struct IPhoneCallVideoCapabilitiesManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, phonenumber: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallVideoCapabilitiesManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089858902, data2: 61451, data3: 18972, data4: [160, 198, 238, 25, 16, 116, 156, 231] };
}
#[repr(C)]
pub struct IPhoneCallsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub OperationStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineOperationStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllActivePhoneCalls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllActivePhoneCalls: usize,
}
impl ::windows_sys::core::Interface for IPhoneCallsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 469422949, data2: 22479, data3: 22493, data4: [152, 109, 176, 87, 201, 30, 172, 51] };
}
#[repr(C)]
pub struct IPhoneDialOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Number: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContact: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContact: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPhone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPhone: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub SetContactPhone: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    SetContactPhone: usize,
    pub Media: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallMedia) -> ::windows_sys::core::HRESULT,
    pub SetMedia: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneCallMedia) -> ::windows_sys::core::HRESULT,
    pub AudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneAudioRoutingEndpoint) -> ::windows_sys::core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(this: *mut *mut Self, value: PhoneAudioRoutingEndpoint) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneDialOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3057239224, data2: 61551, data3: 14027, data4: [168, 99, 130, 55, 66, 181, 242, 212] };
}
#[repr(C)]
pub struct IPhoneLine {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub LineChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineChanged: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub NetworkState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneNetworkState) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Voicemail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NetworkName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CellularDetails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineTransport) -> ::windows_sys::core::HRESULT,
    pub CanDial: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SupportsTile: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub VideoCallingCapabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LineConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsImmediateDialNumberAsync: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsImmediateDialNumberAsync: usize,
    pub Dial: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DialWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 667316016, data2: 27241, data3: 13514, data4: [162, 186, 101, 48, 37, 48, 195, 17] };
}
#[repr(C)]
pub struct IPhoneLine2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EnableTextReply: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub TransportDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLine2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 23590250, data2: 21316, data3: 23908, data4: [138, 243, 163, 26, 149, 14, 145, 106] };
}
#[repr(C)]
pub struct IPhoneLine3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub DialWithResult: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialWithResultAsync: unsafe extern "system" fn(this: *mut *mut Self, number: ::windows_sys::core::HSTRING, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialWithResultAsync: usize,
    pub GetAllActivePhoneCalls: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAllActivePhoneCallsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAllActivePhoneCallsAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneLine3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3806543095, data2: 9222, data3: 22515, data4: [130, 106, 229, 165, 244, 13, 111, 181] };
}
#[repr(C)]
pub struct IPhoneLineCellularDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub SimState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneSimState) -> ::windows_sys::core::HRESULT,
    pub SimSlotIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsModemOn: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub RegistrationRejectCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetNetworkOperatorDisplayText: unsafe extern "system" fn(this: *mut *mut Self, location: PhoneLineNetworkOperatorDisplayTextLocation, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineCellularDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 421921237, data2: 5244, data3: 18281, data4: [182, 115, 152, 165, 236, 132, 38, 203] };
}
#[repr(C)]
pub struct IPhoneLineConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsVideoCallingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
impl ::windows_sys::core::Interface for IPhoneLineConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4263925858, data2: 63055, data3: 17170, data4: [178, 168, 78, 37, 119, 33, 170, 149] };
}
#[repr(C)]
pub struct IPhoneLineDialResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub DialCallStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneCallOperationStatus) -> ::windows_sys::core::HRESULT,
    pub DialedCall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineDialResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3894780682, data2: 23679, data3: 21615, data4: [185, 24, 58, 210, 254, 112, 251, 52] };
}
#[repr(C)]
pub struct IPhoneLineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, lineid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneLineStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4085997347, data2: 52912, data3: 16463, data4: [188, 242, 186, 159, 105, 125, 138, 223] };
}
#[repr(C)]
pub struct IPhoneLineTransportDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Transport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineTransport) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    pub RegisterApp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub RegisterAppForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    RegisterAppForUser: usize,
    pub UnregisterApp: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub UnregisterAppForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    UnregisterAppForUser: usize,
    pub IsRegistered: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneLineTransportDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4020828297, data2: 53242, data3: 23028, data4: [151, 228, 116, 112, 91, 125, 196, 144] };
}
#[repr(C)]
pub struct IPhoneLineTransportDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AudioRoutingStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TransportDeviceAudioRoutingStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioRoutingStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioRoutingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioRoutingStatusChanged: usize,
    pub InBandRingingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InBandRingingEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInBandRingingEnabledChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInBandRingingEnabledChanged: usize,
}
impl ::windows_sys::core::Interface for IPhoneLineTransportDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1690863090, data2: 60660, data3: 22369, data4: [140, 4, 60, 36, 140, 230, 22, 144] };
}
#[repr(C)]
pub struct IPhoneLineTransportDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorForPhoneLineTransport: unsafe extern "system" fn(this: *mut *mut Self, transport: PhoneLineTransport, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineTransportDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 254878124, data2: 54793, data3: 20897, data4: [150, 243, 251, 0, 209, 129, 146, 82] };
}
#[repr(C)]
pub struct IPhoneLineWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LineAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineAdded: usize,
    #[cfg(feature = "Foundation")]
    pub LineRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub LineUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLineUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLineUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneLineWatcherStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2319830282, data2: 25379, data3: 17632, data4: [166, 246, 159, 33, 246, 77, 201, 10] };
}
#[repr(C)]
pub struct IPhoneLineWatcherEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LineId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhoneLineWatcherEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497817406, data2: 40466, data3: 18999, data4: [130, 183, 173, 83, 93, 173, 106, 103] };
}
#[repr(C)]
pub struct IPhoneVoicemail {
    pub base__: ::windows_sys::core::IInspectable,
    pub Number: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhoneVoicemailType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DialVoicemailAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DialVoicemailAsync: usize,
}
impl ::windows_sys::core::Interface for IPhoneVoicemail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3385751542, data2: 28319, data3: 14987, data4: [183, 39, 110, 12, 246, 153, 130, 36] };
}
#[repr(C)]
pub struct IVoipCallCoordinator {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReserveCallResourcesAsync: unsafe extern "system" fn(this: *mut *mut Self, taskentrypoint: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveCallResourcesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MuteStateChanged: unsafe extern "system" fn(this: *mut *mut Self, mutechangehandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCall: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, contactnumber: ::windows_sys::core::HSTRING, contactimage: *mut ::core::ffi::c_void, servicename: ::windows_sys::core::HSTRING, brandingimage: *mut ::core::ffi::c_void, calldetails: ::windows_sys::core::HSTRING, ringtone: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCall: usize,
    pub RequestNewOutgoingCall: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, servicename: ::windows_sys::core::HSTRING, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NotifyMuted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyUnmuted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RequestOutgoingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut *mut Self, callupgradeguid: ::windows_sys::core::GUID, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, servicename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestIncomingUpgradeToVideoCall: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, contactnumber: ::windows_sys::core::HSTRING, contactimage: *mut ::core::ffi::c_void, servicename: ::windows_sys::core::HSTRING, brandingimage: *mut ::core::ffi::c_void, calldetails: ::windows_sys::core::HSTRING, ringtone: *mut ::core::ffi::c_void, ringtimeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestIncomingUpgradeToVideoCall: usize,
    pub TerminateCellularCall: unsafe extern "system" fn(this: *mut *mut Self, callupgradeguid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub CancelUpgrade: unsafe extern "system" fn(this: *mut *mut Self, callupgradeguid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipCallCoordinator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1326549967, data2: 59631, data3: 17460, data4: [156, 95, 168, 216, 147, 250, 254, 121] };
}
#[repr(C)]
pub struct IVoipCallCoordinator2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetupNewAcceptedCall: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, contactnumber: ::windows_sys::core::HSTRING, servicename: ::windows_sys::core::HSTRING, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipCallCoordinator2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3199511027, data2: 50948, data3: 16948, data4: [137, 206, 232, 140, 192, 210, 143, 190] };
}
#[repr(C)]
pub struct IVoipCallCoordinator3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RequestNewAppInitiatedCall: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, contactnumber: ::windows_sys::core::HSTRING, servicename: ::windows_sys::core::HSTRING, media: VoipPhoneCallMedia, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestNewIncomingCallWithContactRemoteId: unsafe extern "system" fn(this: *mut *mut Self, context: ::windows_sys::core::HSTRING, contactname: ::windows_sys::core::HSTRING, contactnumber: ::windows_sys::core::HSTRING, contactimage: *mut ::core::ffi::c_void, servicename: ::windows_sys::core::HSTRING, brandingimage: *mut ::core::ffi::c_void, calldetails: ::windows_sys::core::HSTRING, ringtone: *mut ::core::ffi::c_void, media: VoipPhoneCallMedia, ringtimeout: super::super::Foundation::TimeSpan, contactremoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewIncomingCallWithContactRemoteId: usize,
}
impl ::windows_sys::core::Interface for IVoipCallCoordinator3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 864881855, data2: 39765, data3: 16417, data4: [135, 202, 230, 75, 155, 214, 102, 199] };
}
#[repr(C)]
pub struct IVoipCallCoordinator4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReserveOneProcessCallResourcesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReserveOneProcessCallResourcesAsync: usize,
}
impl ::windows_sys::core::Interface for IVoipCallCoordinator4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2205381177, data2: 37649, data3: 18063, data4: [187, 73, 71, 224, 223, 181, 217, 62] };
}
#[repr(C)]
pub struct IVoipCallCoordinatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipCallCoordinatorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2136809259, data2: 57418, data3: 19728, data4: [179, 26, 165, 92, 146, 44, 194, 251] };
}
#[repr(C)]
pub struct IVoipPhoneCall {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub EndRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEndRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEndRequested: usize,
    #[cfg(feature = "Foundation")]
    pub HoldRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHoldRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHoldRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResumeRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResumeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, accepthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnswerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnswerRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RejectRequested: unsafe extern "system" fn(this: *mut *mut Self, rejecthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RejectRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRejectRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRejectRequested: usize,
    pub NotifyCallHeld: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyCallActive: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub NotifyCallEnded: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub ContactName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContactName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    pub CallMedia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut VoipPhoneCallMedia) -> ::windows_sys::core::HRESULT,
    pub SetCallMedia: unsafe extern "system" fn(this: *mut *mut Self, value: VoipPhoneCallMedia) -> ::windows_sys::core::HRESULT,
    pub NotifyCallReady: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipPhoneCall {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1827795354, data2: 30612, data3: 19034, data4: [140, 104, 174, 135, 148, 122, 105, 144] };
}
#[repr(C)]
pub struct IVoipPhoneCall2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryShowAppUI: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipPhoneCall2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1947944673, data2: 9311, data3: 16883, data4: [147, 153, 49, 65, 210, 91, 82, 227] };
}
#[repr(C)]
pub struct IVoipPhoneCall3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub NotifyCallAccepted: unsafe extern "system" fn(this: *mut *mut Self, media: VoipPhoneCallMedia) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVoipPhoneCall3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 227087650, data2: 57944, data3: 19113, data4: [144, 122, 26, 164, 19, 194, 85, 35] };
}
pub type LockScreenCallEndCallDeferral = *mut ::core::ffi::c_void;
pub type LockScreenCallEndRequestedEventArgs = *mut ::core::ffi::c_void;
pub type LockScreenCallUI = *mut ::core::ffi::c_void;
pub type MuteChangeEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneAudioRoutingEndpoint(pub i32);
impl PhoneAudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Bluetooth: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneAudioRoutingEndpoint {}
impl ::core::clone::Clone for PhoneAudioRoutingEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCall = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallAudioDevice(pub i32);
impl PhoneCallAudioDevice {
    pub const Unknown: Self = Self(0i32);
    pub const LocalDevice: Self = Self(1i32);
    pub const RemoteDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallAudioDevice {}
impl ::core::clone::Clone for PhoneCallAudioDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Unknown: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Outgoing: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallDirection {}
impl ::core::clone::Clone for PhoneCallDirection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallHistoryEntry = *mut ::core::ffi::c_void;
pub type PhoneCallHistoryEntryAddress = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryMedia(pub i32);
impl PhoneCallHistoryEntryMedia {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryOtherAppReadAccess(pub i32);
impl PhoneCallHistoryEntryOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryOtherAppReadAccess {}
impl ::core::clone::Clone for PhoneCallHistoryEntryOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryQueryDesiredMedia(pub u32);
impl PhoneCallHistoryEntryQueryDesiredMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryQueryDesiredMedia {}
impl ::core::clone::Clone for PhoneCallHistoryEntryQueryDesiredMedia {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallHistoryEntryQueryOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryEntryRawAddressKind(pub i32);
impl PhoneCallHistoryEntryRawAddressKind {
    pub const PhoneNumber: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistoryEntryRawAddressKind {}
impl ::core::clone::Clone for PhoneCallHistoryEntryRawAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallHistoryEntryReader = *mut ::core::ffi::c_void;
pub type PhoneCallHistoryManagerForUser = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistorySourceIdKind(pub i32);
impl PhoneCallHistorySourceIdKind {
    pub const CellularPhoneLineId: Self = Self(0i32);
    pub const PackageFamilyName: Self = Self(1i32);
}
impl ::core::marker::Copy for PhoneCallHistorySourceIdKind {}
impl ::core::clone::Clone for PhoneCallHistorySourceIdKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallHistoryStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallHistoryStoreAccessType(pub i32);
impl PhoneCallHistoryStoreAccessType {
    pub const AppEntriesReadWrite: Self = Self(0i32);
    pub const AllEntriesLimitedReadWrite: Self = Self(1i32);
    pub const AllEntriesReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallHistoryStoreAccessType {}
impl ::core::clone::Clone for PhoneCallHistoryStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallMedia(pub i32);
impl PhoneCallMedia {
    pub const Audio: Self = Self(0i32);
    pub const AudioAndVideo: Self = Self(1i32);
    pub const AudioAndRealTimeText: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneCallMedia {}
impl ::core::clone::Clone for PhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallOperationStatus(pub i32);
impl PhoneCallOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneCallOperationStatus {}
impl ::core::clone::Clone for PhoneCallOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneCallStatus(pub i32);
impl PhoneCallStatus {
    pub const Lost: Self = Self(0i32);
    pub const Incoming: Self = Self(1i32);
    pub const Dialing: Self = Self(2i32);
    pub const Talking: Self = Self(3i32);
    pub const Held: Self = Self(4i32);
    pub const Ended: Self = Self(5i32);
}
impl ::core::marker::Copy for PhoneCallStatus {}
impl ::core::clone::Clone for PhoneCallStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneCallStore = *mut ::core::ffi::c_void;
pub type PhoneCallVideoCapabilities = *mut ::core::ffi::c_void;
pub type PhoneCallsResult = *mut ::core::ffi::c_void;
pub type PhoneDialOptions = *mut ::core::ffi::c_void;
pub type PhoneLine = *mut ::core::ffi::c_void;
pub type PhoneLineCellularDetails = *mut ::core::ffi::c_void;
pub type PhoneLineConfiguration = *mut ::core::ffi::c_void;
pub type PhoneLineDialResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineNetworkOperatorDisplayTextLocation(pub i32);
impl PhoneLineNetworkOperatorDisplayTextLocation {
    pub const Default: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Dialer: Self = Self(2i32);
    pub const InCallUI: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineNetworkOperatorDisplayTextLocation {}
impl ::core::clone::Clone for PhoneLineNetworkOperatorDisplayTextLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineOperationStatus(pub i32);
impl PhoneLineOperationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
    pub const ConnectionLost: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneLineOperationStatus {}
impl ::core::clone::Clone for PhoneLineOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineTransport(pub i32);
impl PhoneLineTransport {
    pub const Cellular: Self = Self(0i32);
    pub const VoipApp: Self = Self(1i32);
    pub const Bluetooth: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneLineTransport {}
impl ::core::clone::Clone for PhoneLineTransport {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneLineTransportDevice = *mut ::core::ffi::c_void;
pub type PhoneLineWatcher = *mut ::core::ffi::c_void;
pub type PhoneLineWatcherEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneLineWatcherStatus(pub i32);
impl PhoneLineWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneLineWatcherStatus {}
impl ::core::clone::Clone for PhoneLineWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneNetworkState(pub i32);
impl PhoneNetworkState {
    pub const Unknown: Self = Self(0i32);
    pub const NoSignal: Self = Self(1i32);
    pub const Deregistered: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const Searching: Self = Self(4i32);
    pub const Home: Self = Self(5i32);
    pub const RoamingInternational: Self = Self(6i32);
    pub const RoamingDomestic: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneNetworkState {}
impl ::core::clone::Clone for PhoneNetworkState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneSimState(pub i32);
impl PhoneSimState {
    pub const Unknown: Self = Self(0i32);
    pub const PinNotRequired: Self = Self(1i32);
    pub const PinUnlocked: Self = Self(2i32);
    pub const PinLocked: Self = Self(3i32);
    pub const PukLocked: Self = Self(4i32);
    pub const NotInserted: Self = Self(5i32);
    pub const Invalid: Self = Self(6i32);
    pub const Disabled: Self = Self(7i32);
}
impl ::core::marker::Copy for PhoneSimState {}
impl ::core::clone::Clone for PhoneSimState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhoneVoicemail = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct PhoneVoicemailType(pub i32);
impl PhoneVoicemailType {
    pub const None: Self = Self(0i32);
    pub const Traditional: Self = Self(1i32);
    pub const Visual: Self = Self(2i32);
}
impl ::core::marker::Copy for PhoneVoicemailType {}
impl ::core::clone::Clone for PhoneVoicemailType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct TransportDeviceAudioRoutingStatus(pub i32);
impl TransportDeviceAudioRoutingStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CanRouteToLocalDevice: Self = Self(1i32);
    pub const CannotRouteToLocalDevice: Self = Self(2i32);
}
impl ::core::marker::Copy for TransportDeviceAudioRoutingStatus {}
impl ::core::clone::Clone for TransportDeviceAudioRoutingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VoipCallCoordinator = *mut ::core::ffi::c_void;
pub type VoipPhoneCall = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCallMedia(pub u32);
impl VoipPhoneCallMedia {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
}
impl ::core::marker::Copy for VoipPhoneCallMedia {}
impl ::core::clone::Clone for VoipPhoneCallMedia {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCallRejectReason(pub i32);
impl VoipPhoneCallRejectReason {
    pub const UserIgnored: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const OtherIncomingCall: Self = Self(2i32);
    pub const EmergencyCallExists: Self = Self(3i32);
    pub const InvalidCallState: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallRejectReason {}
impl ::core::clone::Clone for VoipPhoneCallRejectReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCallResourceReservationStatus(pub i32);
impl VoipPhoneCallResourceReservationStatus {
    pub const Success: Self = Self(0i32);
    pub const ResourcesNotAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for VoipPhoneCallResourceReservationStatus {}
impl ::core::clone::Clone for VoipPhoneCallResourceReservationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
#[repr(transparent)]
pub struct VoipPhoneCallState(pub i32);
impl VoipPhoneCallState {
    pub const Ended: Self = Self(0i32);
    pub const Held: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Incoming: Self = Self(3i32);
    pub const Outgoing: Self = Self(4i32);
}
impl ::core::marker::Copy for VoipPhoneCallState {}
impl ::core::clone::Clone for VoipPhoneCallState {
    fn clone(&self) -> Self {
        *self
    }
}
