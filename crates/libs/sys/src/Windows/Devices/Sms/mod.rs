#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: Self = Self(0i32);
    pub const Gsm: Self = Self(1i32);
    pub const Cdma: Self = Self(2i32);
}
impl ::core::marker::Copy for CellularClass {}
impl ::core::clone::Clone for CellularClass {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DeleteSmsMessageOperation = *mut ::core::ffi::c_void;
pub type DeleteSmsMessagesOperation = *mut ::core::ffi::c_void;
pub type GetSmsDeviceOperation = *mut ::core::ffi::c_void;
pub type GetSmsMessageOperation = *mut ::core::ffi::c_void;
pub type GetSmsMessagesOperation = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ISmsAppMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsEncoding) -> ::windows_sys::core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut *mut Self, value: SmsEncoding) -> ::windows_sys::core::HRESULT,
    pub PortNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPortNumber: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTeleserviceId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetProtocolId: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBinaryBody: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBinaryBody: usize,
}
impl ::windows_sys::core::Interface for ISmsAppMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3904603284, data2: 54176, data3: 18954, data4: [134, 215, 41, 16, 51, 168, 207, 84] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsBinaryMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsDataFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Format: usize,
    #[cfg(feature = "deprecated")]
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, value: SmsDataFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFormat: usize,
    #[cfg(feature = "deprecated")]
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetData: usize,
    #[cfg(feature = "deprecated")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsBinaryMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1542776851, data2: 15187, data3: 19566, data4: [182, 26, 216, 106, 99, 117, 86, 80] };
}
#[repr(C)]
pub struct ISmsBroadcastMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GeographicalScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsGeographicalScope) -> ::windows_sys::core::HRESULT,
    pub MessageCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub UpdateNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub BroadcastType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsBroadcastType) -> ::windows_sys::core::HRESULT,
    pub IsEmergencyAlert: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsUserPopupRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsBroadcastMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1974385649, data2: 58551, data3: 18548, data4: [160, 156, 41, 86, 229, 146, 249, 87] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendMessageAsync: usize,
    #[cfg(feature = "deprecated")]
    pub CalculateLength: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CalculateLength: usize,
    #[cfg(feature = "deprecated")]
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AccountPhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularClass) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CellularClass: usize,
    #[cfg(feature = "deprecated")]
    pub MessageStore: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageStore: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsDeviceStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceStatus: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmsMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmsMessageReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSmsMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSmsMessageReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmsDeviceStatusChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSmsDeviceStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSmsDeviceStatusChanged: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 152539629, data2: 34603, data3: 20204, data4: [156, 114, 171, 17, 98, 123, 52, 236] };
}
#[repr(C)]
pub struct ISmsDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SmscAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSmscAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AccountPhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularClass) -> ::windows_sys::core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsDeviceStatus) -> ::windows_sys::core::HRESULT,
    pub CalculateLength: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut SmsEncodedLength) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendMessageAndGetResultAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAndGetResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceStatusChanged: usize,
}
impl ::windows_sys::core::Interface for ISmsDevice2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3179961363, data2: 58658, data3: 18123, data4: [184, 213, 158, 173, 48, 251, 108, 71] };
}
#[repr(C)]
pub struct ISmsDevice2Statics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FromParentId: unsafe extern "system" fn(this: *mut *mut Self, parentdeviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsDevice2Statics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1707574053, data2: 4145, data3: 18718, data4: [143, 182, 239, 153, 145, 175, 227, 99] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsDeviceMessageStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeleteMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeleteMessagesAsync: unsafe extern "system" fn(this: *mut *mut Self, messagefilter: SmsMessageFilter, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeleteMessagesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetMessageAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetMessagesAsync: unsafe extern "system" fn(this: *mut *mut Self, messagefilter: SmsMessageFilter, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    GetMessagesAsync: usize,
    #[cfg(feature = "deprecated")]
    pub MaxMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxMessages: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsDeviceMessageStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2559177299, data2: 61832, data3: 17447, data4: [141, 84, 206, 12, 36, 35, 197, 193] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeviceSelector: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDefaultAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169992170, data2: 55317, data3: 19921, data4: [162, 52, 69, 32, 206, 70, 4, 164] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsDeviceStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromNetworkAccountIdAsync: unsafe extern "system" fn(this: *mut *mut Self, networkaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromNetworkAccountIdAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsDeviceStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 748756103, data2: 2163, data3: 19631, data4: [138, 125, 189, 71, 30, 133, 134, 209] };
}
#[repr(C)]
pub struct ISmsFilterRule {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ImsiPrefixes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImsiPrefixes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SenderNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SenderNumbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TextMessagePrefixes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextMessagePrefixes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PortNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PortNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularClass) -> ::windows_sys::core::HRESULT,
    pub SetCellularClass: unsafe extern "system" fn(this: *mut *mut Self, value: CellularClass) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProtocolIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProtocolIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TeleserviceIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TeleserviceIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WapApplicationIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WapApplicationIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WapContentTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WapContentTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastChannels: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastChannels: usize,
}
impl ::windows_sys::core::Interface for ISmsFilterRule {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1088630702, data2: 45129, data3: 20412, data4: [175, 233, 226, 166, 16, 239, 245, 92] };
}
#[repr(C)]
pub struct ISmsFilterRuleFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFilterRule: unsafe extern "system" fn(this: *mut *mut Self, messagetype: SmsMessageType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsFilterRuleFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 12805384, data2: 25238, data3: 20265, data4: [154, 173, 137, 32, 206, 186, 60, 232] };
}
#[repr(C)]
pub struct ISmsFilterRules {
    pub base__: ::windows_sys::core::IInspectable,
    pub ActionType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsFilterActionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Rules: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Rules: usize,
}
impl ::windows_sys::core::Interface for ISmsFilterRules {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1313336059, data2: 31181, data3: 18561, data4: [152, 148, 85, 164, 19, 91, 35, 250] };
}
#[repr(C)]
pub struct ISmsFilterRulesFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFilterRules: unsafe extern "system" fn(this: *mut *mut Self, actiontype: SmsFilterActionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsFilterRulesFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2694391021, data2: 28206, data3: 17712, data4: [159, 222, 70, 93, 2, 238, 208, 14] };
}
#[repr(C)]
pub struct ISmsMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageClass) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3980156456, data2: 27012, data3: 19207, data4: [129, 29, 141, 89, 6, 237, 60, 234] };
}
#[repr(C)]
pub struct ISmsMessageBase {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageType) -> ::windows_sys::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularClass) -> ::windows_sys::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageClass) -> ::windows_sys::core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsMessageBase {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 753991216, data2: 65104, data3: 20422, data4: [170, 136, 76, 207, 226, 122, 41, 234] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsMessageReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub TextMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TextMessage: usize,
    #[cfg(feature = "deprecated")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BinaryMessage: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsMessageReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 149424792, data2: 47333, data3: 16833, data4: [163, 216, 211, 171, 250, 226, 38, 117] };
}
#[repr(C)]
pub struct ISmsMessageReceivedTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub MessageType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageType) -> ::windows_sys::core::HRESULT,
    pub TextMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WapMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub BroadcastMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VoicemailMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StatusMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Drop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsMessageReceivedTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 735038420, data2: 9815, data3: 16680, data4: [173, 95, 227, 135, 113, 50, 189, 177] };
}
#[repr(C)]
pub struct ISmsMessageRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageReceived: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageReceived: usize,
}
impl ::windows_sys::core::Interface for ISmsMessageRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 387993662, data2: 62287, data3: 17515, data4: [131, 179, 15, 241, 153, 35, 180, 9] };
}
#[repr(C)]
pub struct ISmsMessageRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AllRegistrations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllRegistrations: usize,
    pub Register: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsMessageRegistrationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1671451748, data2: 10392, data3: 18296, data4: [160, 60, 111, 153, 73, 7, 214, 58] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsReceivedEventDetails {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub MessageIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageIndex: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsReceivedEventDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1538592533, data2: 58477, data3: 19586, data4: [132, 125, 90, 3, 4, 193, 213, 61] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsReceivedEventDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub MessageClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsMessageClass) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MessageClass: usize,
    #[cfg(feature = "deprecated")]
    pub BinaryMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BinaryMessage: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsReceivedEventDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1088445574, data2: 42932, data3: 18289, data4: [154, 231, 11, 95, 251, 18, 192, 58] };
}
#[repr(C)]
pub struct ISmsSendMessageResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageReferenceNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageReferenceNumbers: usize,
    pub CellularClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CellularClass) -> ::windows_sys::core::HRESULT,
    pub ModemErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsModemErrorCode) -> ::windows_sys::core::HRESULT,
    pub IsErrorTransient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub NetworkCauseCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub TransportFailureCause: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsSendMessageResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3675495154, data2: 30921, data3: 20459, data4: [150, 34, 69, 35, 40, 8, 141, 98] };
}
#[repr(C)]
pub struct ISmsStatusMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MessageReferenceNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceCenterTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceCenterTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub DischargeTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DischargeTime: usize,
}
impl ::windows_sys::core::Interface for ISmsStatusMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3872555842, data2: 46859, data3: 18039, data4: [147, 121, 201, 120, 63, 223, 248, 244] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsTextMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(feature = "deprecated")]
    pub PartReferenceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartReferenceId: usize,
    #[cfg(feature = "deprecated")]
    pub PartNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartNumber: usize,
    #[cfg(feature = "deprecated")]
    pub PartCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PartCount: usize,
    #[cfg(feature = "deprecated")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    To: usize,
    #[cfg(feature = "deprecated")]
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTo: usize,
    #[cfg(feature = "deprecated")]
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    From: usize,
    #[cfg(feature = "deprecated")]
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFrom: usize,
    #[cfg(feature = "deprecated")]
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Body: usize,
    #[cfg(feature = "deprecated")]
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBody: usize,
    #[cfg(feature = "deprecated")]
    pub Encoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsEncoding) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Encoding: usize,
    #[cfg(feature = "deprecated")]
    pub SetEncoding: unsafe extern "system" fn(this: *mut *mut Self, value: SmsEncoding) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetEncoding: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ToBinaryMessages: unsafe extern "system" fn(this: *mut *mut Self, format: SmsDataFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ToBinaryMessages: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsTextMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3592196172, data2: 42133, data3: 18559, data4: [154, 111, 151, 21, 72, 197, 188, 159] };
}
#[repr(C)]
pub struct ISmsTextMessage2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Encoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SmsEncoding) -> ::windows_sys::core::HRESULT,
    pub SetEncoding: unsafe extern "system" fn(this: *mut *mut Self, value: SmsEncoding) -> ::windows_sys::core::HRESULT,
    pub CallbackNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCallbackNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDeliveryNotificationEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RetryAttemptCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRetryAttemptCount: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub TeleserviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ProtocolId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISmsTextMessage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 580966547, data2: 17749, data3: 18261, data4: [181, 161, 231, 253, 132, 149, 95, 141] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISmsTextMessageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub FromBinaryMessage: unsafe extern "system" fn(this: *mut *mut Self, binarymessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FromBinaryMessage: usize,
    #[cfg(feature = "deprecated")]
    pub FromBinaryData: unsafe extern "system" fn(this: *mut *mut Self, format: SmsDataFormat, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FromBinaryData: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISmsTextMessageStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2137572845, data2: 15564, data3: 18339, data4: [140, 85, 56, 13, 59, 1, 8, 146] };
}
#[repr(C)]
pub struct ISmsVoicemailMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageCount: usize,
}
impl ::windows_sys::core::Interface for ISmsVoicemailMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 656056486, data2: 38321, data3: 17663, data4: [188, 184, 184, 253, 215, 224, 139, 195] };
}
#[repr(C)]
pub struct ISmsWapMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub BinaryBody: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BinaryBody: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
}
impl ::windows_sys::core::Interface for ISmsWapMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3448993603, data2: 31317, data3: 19771, data4: [144, 33, 242, 46, 2, 45, 9, 197] };
}
pub type SendSmsMessageOperation = *mut ::core::ffi::c_void;
pub type SmsAppMessage = *mut ::core::ffi::c_void;
pub type SmsBinaryMessage = *mut ::core::ffi::c_void;
pub type SmsBroadcastMessage = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: Self = Self(0i32);
    pub const CmasPresidential: Self = Self(1i32);
    pub const CmasExtreme: Self = Self(2i32);
    pub const CmasSevere: Self = Self(3i32);
    pub const CmasAmber: Self = Self(4i32);
    pub const CmasTest: Self = Self(5i32);
    pub const EUAlert1: Self = Self(6i32);
    pub const EUAlert2: Self = Self(7i32);
    pub const EUAlert3: Self = Self(8i32);
    pub const EUAlertAmber: Self = Self(9i32);
    pub const EUAlertInfo: Self = Self(10i32);
    pub const EtwsEarthquake: Self = Self(11i32);
    pub const EtwsTsunami: Self = Self(12i32);
    pub const EtwsTsunamiAndEarthquake: Self = Self(13i32);
    pub const LatAlertLocal: Self = Self(14i32);
}
impl ::core::marker::Copy for SmsBroadcastType {}
impl ::core::clone::Clone for SmsBroadcastType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: Self = Self(0i32);
    pub const CdmaSubmit: Self = Self(1i32);
    pub const GsmSubmit: Self = Self(2i32);
    pub const CdmaDeliver: Self = Self(3i32);
    pub const GsmDeliver: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsDataFormat {}
impl ::core::clone::Clone for SmsDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmsDevice = *mut ::core::ffi::c_void;
pub type SmsDevice2 = *mut ::core::ffi::c_void;
pub type SmsDeviceMessageStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const SubscriptionNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for SmsDeviceStatus {}
impl ::core::clone::Clone for SmsDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmsDeviceStatusChangedEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Devices_Sms\"`*"]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl ::core::marker::Copy for SmsEncodedLength {}
impl ::core::clone::Clone for SmsEncodedLength {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: Self = Self(0i32);
    pub const Optimal: Self = Self(1i32);
    pub const SevenBitAscii: Self = Self(2i32);
    pub const Unicode: Self = Self(3i32);
    pub const GsmSevenBit: Self = Self(4i32);
    pub const EightBit: Self = Self(5i32);
    pub const Latin: Self = Self(6i32);
    pub const Korean: Self = Self(7i32);
    pub const IA5: Self = Self(8i32);
    pub const ShiftJis: Self = Self(9i32);
    pub const LatinHebrew: Self = Self(10i32);
}
impl ::core::marker::Copy for SmsEncoding {}
impl ::core::clone::Clone for SmsEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: Self = Self(0i32);
    pub const Drop: Self = Self(1i32);
    pub const Peek: Self = Self(2i32);
    pub const Accept: Self = Self(3i32);
}
impl ::core::marker::Copy for SmsFilterActionType {}
impl ::core::clone::Clone for SmsFilterActionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmsFilterRule = *mut ::core::ffi::c_void;
pub type SmsFilterRules = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: Self = Self(0i32);
    pub const CellWithImmediateDisplay: Self = Self(1i32);
    pub const LocationArea: Self = Self(2i32);
    pub const Plmn: Self = Self(3i32);
    pub const Cell: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsGeographicalScope {}
impl ::core::clone::Clone for SmsGeographicalScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: Self = Self(0i32);
    pub const Class0: Self = Self(1i32);
    pub const Class1: Self = Self(2i32);
    pub const Class2: Self = Self(3i32);
    pub const Class3: Self = Self(4i32);
}
impl ::core::marker::Copy for SmsMessageClass {}
impl ::core::clone::Clone for SmsMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SmsMessageFilter(pub i32);
#[cfg(feature = "deprecated")]
impl SmsMessageFilter {
    pub const All: Self = Self(0i32);
    pub const Unread: Self = Self(1i32);
    pub const Read: Self = Self(2i32);
    pub const Sent: Self = Self(3i32);
    pub const Draft: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SmsMessageFilter {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SmsMessageFilter {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmsMessageReceivedEventArgs = *mut ::core::ffi::c_void;
pub type SmsMessageReceivedEventHandler = *mut ::core::ffi::c_void;
pub type SmsMessageReceivedTriggerDetails = *mut ::core::ffi::c_void;
pub type SmsMessageRegistration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
    pub const Wap: Self = Self(2i32);
    pub const App: Self = Self(3i32);
    pub const Broadcast: Self = Self(4i32);
    pub const Voicemail: Self = Self(5i32);
    pub const Status: Self = Self(6i32);
}
impl ::core::marker::Copy for SmsMessageType {}
impl ::core::clone::Clone for SmsMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Sms\"`*"]
#[repr(transparent)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: Self = Self(0i32);
    pub const MessagingNetworkError: Self = Self(1i32);
    pub const SmsOperationNotSupportedByDevice: Self = Self(2i32);
    pub const SmsServiceNotSupportedByNetwork: Self = Self(3i32);
    pub const DeviceFailure: Self = Self(4i32);
    pub const MessageNotEncodedProperly: Self = Self(5i32);
    pub const MessageTooLarge: Self = Self(6i32);
    pub const DeviceNotReady: Self = Self(7i32);
    pub const NetworkNotReady: Self = Self(8i32);
    pub const InvalidSmscAddress: Self = Self(9i32);
    pub const NetworkFailure: Self = Self(10i32);
    pub const FixedDialingNumberRestricted: Self = Self(11i32);
}
impl ::core::marker::Copy for SmsModemErrorCode {}
impl ::core::clone::Clone for SmsModemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SmsReceivedEventDetails = *mut ::core::ffi::c_void;
pub type SmsSendMessageResult = *mut ::core::ffi::c_void;
pub type SmsStatusMessage = *mut ::core::ffi::c_void;
pub type SmsTextMessage = *mut ::core::ffi::c_void;
pub type SmsTextMessage2 = *mut ::core::ffi::c_void;
pub type SmsVoicemailMessage = *mut ::core::ffi::c_void;
pub type SmsWapMessage = *mut ::core::ffi::c_void;
