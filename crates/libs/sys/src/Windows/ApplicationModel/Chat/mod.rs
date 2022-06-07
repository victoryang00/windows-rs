pub type ChatCapabilities = *mut ::core::ffi::c_void;
pub type ChatConversation = *mut ::core::ffi::c_void;
pub type ChatConversationReader = *mut ::core::ffi::c_void;
pub type ChatConversationThreadingInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatConversationThreadingKind {}
impl ::core::clone::Clone for ChatConversationThreadingKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
}
impl ::core::marker::Copy for ChatItemKind {}
impl ::core::clone::Clone for ChatItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessage = *mut ::core::ffi::c_void;
pub type ChatMessageAttachment = *mut ::core::ffi::c_void;
pub type ChatMessageChange = *mut ::core::ffi::c_void;
pub type ChatMessageChangeReader = *mut ::core::ffi::c_void;
pub type ChatMessageChangeTracker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageChangeType {}
impl ::core::clone::Clone for ChatMessageChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessageChangedDeferral = *mut ::core::ffi::c_void;
pub type ChatMessageChangedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: Self = Self(0i32);
    pub const FileTransferRequest: Self = Self(1i32);
    pub const TransportCustom: Self = Self(2i32);
    pub const JoinedConversation: Self = Self(3i32);
    pub const LeftConversation: Self = Self(4i32);
    pub const OtherParticipantJoinedConversation: Self = Self(5i32);
    pub const OtherParticipantLeftConversation: Self = Self(6i32);
}
impl ::core::marker::Copy for ChatMessageKind {}
impl ::core::clone::Clone for ChatMessageKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessageNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageOperatorKind {}
impl ::core::clone::Clone for ChatMessageOperatorKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessageReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: Self = Self(0i32);
    pub const Sending: Self = Self(1i32);
    pub const Sent: Self = Self(2i32);
    pub const SendRetryNeeded: Self = Self(3i32);
    pub const SendFailed: Self = Self(4i32);
    pub const Received: Self = Self(5i32);
    pub const ReceiveDownloadNeeded: Self = Self(6i32);
    pub const ReceiveDownloadFailed: Self = Self(7i32);
    pub const ReceiveDownloading: Self = Self(8i32);
    pub const Deleted: Self = Self(9i32);
    pub const Declined: Self = Self(10i32);
    pub const Cancelled: Self = Self(11i32);
    pub const Recalled: Self = Self(12i32);
    pub const ReceiveRetryNeeded: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageStatus {}
impl ::core::clone::Clone for ChatMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessageStore = *mut ::core::ffi::c_void;
pub type ChatMessageStoreChangedEventArgs = *mut ::core::ffi::c_void;
pub type ChatMessageTransport = *mut ::core::ffi::c_void;
pub type ChatMessageTransportConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageTransportKind {}
impl ::core::clone::Clone for ChatMessageTransportKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatMessageValidationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: Self = Self(0i32);
    pub const NoRecipients: Self = Self(1i32);
    pub const InvalidData: Self = Self(2i32);
    pub const MessageTooLarge: Self = Self(3i32);
    pub const TooManyRecipients: Self = Self(4i32);
    pub const TransportInactive: Self = Self(5i32);
    pub const TransportNotFound: Self = Self(6i32);
    pub const TooManyAttachments: Self = Self(7i32);
    pub const InvalidRecipients: Self = Self(8i32);
    pub const InvalidBody: Self = Self(9i32);
    pub const InvalidOther: Self = Self(10i32);
    pub const ValidWithLargeMessage: Self = Self(11i32);
    pub const VoiceRoamingRestriction: Self = Self(12i32);
    pub const DataRoamingRestriction: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageValidationStatus {}
impl ::core::clone::Clone for ChatMessageValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatQueryOptions = *mut ::core::ffi::c_void;
pub type ChatRecipientDeliveryInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
impl ::core::marker::Copy for ChatRestoreHistorySpan {}
impl ::core::clone::Clone for ChatRestoreHistorySpan {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatSearchReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: Self = Self(0i32);
    pub const StoreModified: Self = Self(1i32);
    pub const MessageCreated: Self = Self(2i32);
    pub const MessageModified: Self = Self(3i32);
    pub const MessageDeleted: Self = Self(4i32);
    pub const ConversationModified: Self = Self(5i32);
    pub const ConversationDeleted: Self = Self(6i32);
    pub const ConversationTransportDeleted: Self = Self(7i32);
}
impl ::core::marker::Copy for ChatStoreChangedEventKind {}
impl ::core::clone::Clone for ChatStoreChangedEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ChatSyncConfiguration = *mut ::core::ffi::c_void;
pub type ChatSyncManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatTransportErrorCodeCategory {}
impl ::core::clone::Clone for ChatTransportErrorCodeCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidRecipientAddress: Self = Self(2i32);
    pub const NetworkConnectivity: Self = Self(3i32);
    pub const ServiceDenied: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
}
impl ::core::marker::Copy for ChatTransportInterpretedErrorCode {}
impl ::core::clone::Clone for ChatTransportInterpretedErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IChatCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOnline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsChatCapable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsFileTransferCapable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsGeoLocationPushCapable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsIntegratedMessagingCapable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 989820860, data2: 14793, data3: 19921, data4: [173, 45, 57, 100, 221, 157, 64, 63] };
}
#[repr(C)]
pub struct IChatCapabilitiesManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkAsync: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkAsync: usize,
}
impl ::windows_sys::core::Interface for IChatCapabilitiesManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3044683568, data2: 28737, data3: 17806, data4: [176, 207, 124, 13, 159, 234, 51, 58] };
}
#[repr(C)]
pub struct IChatCapabilitiesManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesForTransportAsync: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, transportid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesForTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkForTransportAsync: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, transportid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkForTransportAsync: usize,
}
impl ::windows_sys::core::Interface for IChatCapabilitiesManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3809297012, data2: 54721, data3: 19145, data4: [159, 252, 64, 230, 145, 132, 254, 200] };
}
#[repr(C)]
pub struct IChatConversation {
    pub base__: ::windows_sys::core::IInspectable,
    pub HasUnreadMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsConversationMuted: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsConversationMuted: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MarkAllMessagesAsReadAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessagesAsReadAsync: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    pub NotifyLocalParticipantComposing: unsafe extern "system" fn(this: *mut *mut Self, transportid: ::windows_sys::core::HSTRING, participantaddress: ::windows_sys::core::HSTRING, iscomposing: bool) -> ::windows_sys::core::HRESULT,
    pub NotifyRemoteParticipantComposing: unsafe extern "system" fn(this: *mut *mut Self, transportid: ::windows_sys::core::HSTRING, participantaddress: ::windows_sys::core::HSTRING, iscomposing: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteParticipantComposingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteParticipantComposingChanged: usize,
}
impl ::windows_sys::core::Interface for IChatConversation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2777417741, data2: 6767, data3: 18140, data4: [143, 61, 245, 2, 134, 96, 182, 238] };
}
#[repr(C)]
pub struct IChatConversation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanModifyParticipants: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanModifyParticipants: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatConversation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 167972049, data2: 38970, data3: 18346, data4: [154, 144, 238, 72, 238, 153, 123, 89] };
}
#[repr(C)]
pub struct IChatConversationReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut *mut Self, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
impl ::windows_sys::core::Interface for IChatConversationReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 89208530, data2: 56882, data3: 19015, data4: [169, 58, 179, 220, 8, 51, 133, 43] };
}
#[repr(C)]
pub struct IChatConversationThreadingInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCustom: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetConversationId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatConversationThreadingKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ChatConversationThreadingKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatConversationThreadingInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 857481692, data2: 31239, data3: 17442, data4: [163, 44, 36, 190, 124, 109, 171, 36] };
}
#[repr(C)]
pub struct IChatItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatItemKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2270285824, data2: 52913, data3: 16963, data4: [184, 3, 21, 212, 90, 29, 212, 40] };
}
#[repr(C)]
pub struct IChatMessage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsForwardingDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocalTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocalTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkTimestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkTimestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientSendStatuses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientSendStatuses: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageStatus) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTransportId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1262028074, data2: 4418, data3: 20617, data4: [118, 218, 242, 219, 61, 23, 205, 5] };
}
#[repr(C)]
pub struct IChatMessage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub EstimatedDownloadSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetEstimatedDownloadSize: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsAutoReply: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAutoReply: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetIsForwardingDisabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsReplyDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSimMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLocalTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLocalTimestamp: usize,
    pub MessageKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageKind) -> ::windows_sys::core::HRESULT,
    pub SetMessageKind: unsafe extern "system" fn(this: *mut *mut Self, value: ChatMessageKind) -> ::windows_sys::core::HRESULT,
    pub MessageOperatorKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageOperatorKind) -> ::windows_sys::core::HRESULT,
    pub SetMessageOperatorKind: unsafe extern "system" fn(this: *mut *mut Self, value: ChatMessageOperatorKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetNetworkTimestamp: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNetworkTimestamp: usize,
    pub IsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: ChatMessageStatus) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ShouldSuppressNotification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetShouldSuppressNotification: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetThreadingInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientsDeliveryInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientsDeliveryInfos: usize,
}
impl ::windows_sys::core::Interface for IChatMessage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2254865202, data2: 21567, data3: 18933, data4: [172, 113, 108, 42, 252, 101, 101, 253] };
}
#[repr(C)]
pub struct IChatMessage3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1961570224, data2: 15271, data3: 17823, data4: [142, 11, 232, 175, 15, 235, 217, 173] };
}
#[repr(C)]
pub struct IChatMessage4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SyncId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSyncId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 756304655, data2: 53951, data3: 17932, data4: [170, 104, 109, 63, 132, 131, 201, 191] };
}
#[repr(C)]
pub struct IChatMessageAttachment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub DataStreamReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DataStreamReference: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDataStreamReference: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDataStreamReference: usize,
    pub GroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageAttachment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3351575924, data2: 48995, data3: 22763, data4: [80, 140, 139, 134, 63, 241, 107, 103] };
}
#[repr(C)]
pub struct IChatMessageAttachment2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub TransferProgress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTransferProgress: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OriginalFileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOriginalFileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageAttachment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1591317104, data2: 32209, data3: 19079, data4: [168, 206, 172, 221, 135, 216, 13, 200] };
}
#[repr(C)]
pub struct IChatMessageAttachmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateChatMessageAttachment: unsafe extern "system" fn(this: *mut *mut Self, mimetype: ::windows_sys::core::HSTRING, datastreamreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateChatMessageAttachment: usize,
}
impl ::windows_sys::core::Interface for IChatMessageAttachmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 542659234, data2: 41814, data3: 23409, data4: [108, 169, 102, 201, 133, 183, 208, 213] };
}
#[repr(C)]
pub struct IChatMessageBlockingStatic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsBlockedAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, blocked: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsBlockedAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageBlockingStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4139361152, data2: 52714, data3: 4580, data4: [136, 48, 8, 0, 32, 12, 154, 102] };
}
#[repr(C)]
pub struct IChatMessageChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageChangeType) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 471384917, data2: 16926, data3: 21688, data4: [109, 56, 107, 58, 108, 130, 252, 204] };
}
#[repr(C)]
pub struct IChatMessageChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut *mut Self, lastchangetoacknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 338063392, data2: 10446, data3: 24358, data4: [123, 5, 154, 92, 124, 206, 135, 202] };
}
#[repr(C)]
pub struct IChatMessageChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageChangeTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1622667366, data2: 28832, data3: 21028, data4: [80, 140, 36, 46, 247, 193, 208, 111] };
}
#[repr(C)]
pub struct IChatMessageChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageChangedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4224103180, data2: 30860, data3: 19916, data4: [172, 231, 98, 130, 56, 41, 104, 207] };
}
#[repr(C)]
pub struct IChatMessageChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3065462317, data2: 26908, data3: 20191, data4: [134, 96, 110, 185, 137, 104, 146, 227] };
}
#[repr(C)]
pub struct IChatMessageManager2Statics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RegisterTransportAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut *mut Self, transportid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageManager2Statics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 491075855, data2: 40783, data3: 20021, data4: [150, 78, 27, 156, 166, 26, 192, 68] };
}
#[repr(C)]
pub struct IChatMessageManagerStatic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowComposeSmsMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeSmsMessageAsync: usize,
    pub ShowSmsSettings: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageManagerStatic {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4049363191, data2: 54760, data3: 24210, data4: [85, 109, 224, 59, 96, 37, 49, 4] };
}
#[repr(C)]
pub struct IChatMessageManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSyncManagerAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 546013965, data2: 26453, data3: 18636, data4: [154, 179, 253, 3, 196, 99, 252, 146] };
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChatMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4248063483, data2: 12387, data3: 19991, data4: [133, 134, 198, 192, 130, 98, 230, 192] };
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldDisplayToast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateDetailText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateBadge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateActionCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageNotificationTriggerDetails2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1807033056, data2: 43527, data3: 20433, data4: [148, 113, 119, 147, 79, 183, 94, 230] };
}
#[repr(C)]
pub struct IChatMessageReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3068819662, data2: 17545, data3: 22265, data4: [118, 170, 226, 4, 104, 37, 20, 207] };
}
#[repr(C)]
pub struct IChatMessageReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut *mut Self, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageReader2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2305046147, data2: 25787, data3: 18189, data4: [157, 244, 13, 232, 190, 26, 5, 191] };
}
#[repr(C)]
pub struct IChatMessageStore {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageReader2: unsafe extern "system" fn(this: *mut *mut Self, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageReader2: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetrySendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrySendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    pub ValidateMessage: unsafe extern "system" fn(this: *mut *mut Self, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageChanged: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageChanged: usize,
}
impl ::windows_sys::core::Interface for IChatMessageStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 838008065, data2: 52470, data3: 22539, data4: [73, 118, 10, 7, 221, 93, 59, 71] };
}
#[repr(C)]
pub struct IChatMessageStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ForwardMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, addresses: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ForwardMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut *mut Self, conversationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsAsync: unsafe extern "system" fn(this: *mut *mut Self, conversationid: ::windows_sys::core::HSTRING, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationFromThreadingInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, threadinginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationFromThreadingInfoAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsReader: unsafe extern "system" fn(this: *mut *mut Self, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsReader: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageByRemoteIdAsync: unsafe extern "system" fn(this: *mut *mut Self, transportid: ::windows_sys::core::HSTRING, remoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnseenCountForTransportsReaderAsync: unsafe extern "system" fn(this: *mut *mut Self, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnseenCountForTransportsReaderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkAsSeenForTransportsAsync: unsafe extern "system" fn(this: *mut *mut Self, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkAsSeenForTransportsAsync: usize,
    pub GetSearchReader: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelDownloadMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelDownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelSendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
}
impl ::windows_sys::core::Interface for IChatMessageStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2907555054, data2: 15060, data3: 18715, data4: [179, 17, 171, 223, 155, 178, 39, 104] };
}
#[repr(C)]
pub struct IChatMessageStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetMessageBySyncIdAsync: unsafe extern "system" fn(this: *mut *mut Self, syncid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageBySyncIdAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageStore3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2598091529, data2: 17221, data3: 20161, data4: [139, 116, 183, 51, 130, 67, 113, 156] };
}
#[repr(C)]
pub struct IChatMessageStoreChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatStoreChangedEventKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageStoreChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1707503532, data2: 65164, data3: 18132, data4: [145, 25, 87, 184, 65, 3, 17, 213] };
}
#[repr(C)]
pub struct IChatMessageTransport {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAppSetAsNotificationProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsNotificationProviderAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsNotificationProviderAsync: usize,
}
impl ::windows_sys::core::Interface for IChatMessageTransport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1672076280, data2: 59059, data3: 23706, data4: [95, 133, 212, 121, 37, 185, 189, 24] };
}
#[repr(C)]
pub struct IChatMessageTransport2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransportKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageTransportKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageTransport2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2426885666, data2: 55370, data3: 19490, data4: [169, 77, 84, 68, 68, 237, 200, 161] };
}
#[repr(C)]
pub struct IChatMessageTransportConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SupportedVideoFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SupportedVideoFormat: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
impl ::windows_sys::core::Interface for IChatMessageTransportConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2275407653, data2: 6664, data3: 19146, data4: [160, 117, 51, 85, 18, 99, 18, 230] };
}
#[repr(C)]
pub struct IChatMessageValidationResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MaxPartCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartCount: usize,
    #[cfg(feature = "Foundation")]
    pub PartCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PartCount: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCharacterCountInPart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCharacterCountInPart: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageValidationStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatMessageValidationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 636041731, data2: 10476, data3: 22665, data4: [86, 155, 126, 72, 107, 18, 111, 24] };
}
#[repr(C)]
pub struct IChatQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSearchString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 802383014, data2: 48950, data3: 17143, data4: [183, 231, 146, 60, 10, 171, 254, 22] };
}
#[repr(C)]
pub struct IChatRecipientDeliveryInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransportAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeliveryTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReadTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReadTime: usize,
    pub TransportErrorCodeCategory: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatTransportErrorCodeCategory) -> ::windows_sys::core::HRESULT,
    pub TransportInterpretedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows_sys::core::HRESULT,
    pub TransportErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub IsErrorPermanent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatRecipientDeliveryInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4291277474, data2: 10300, data3: 19466, data4: [138, 14, 140, 51, 189, 191, 5, 69] };
}
#[repr(C)]
pub struct IChatSearchReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut *mut Self, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
impl ::windows_sys::core::Interface for IChatSearchReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1181089353, data2: 36896, data3: 18258, data4: [152, 13, 57, 97, 35, 37, 245, 137] };
}
#[repr(C)]
pub struct IChatSyncConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RestoreHistorySpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatRestoreHistorySpan) -> ::windows_sys::core::HRESULT,
    pub SetRestoreHistorySpan: unsafe extern "system" fn(this: *mut *mut Self, value: ChatRestoreHistorySpan) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IChatSyncConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 167274930, data2: 27124, data3: 19199, data4: [130, 182, 6, 153, 47, 244, 2, 210] };
}
#[repr(C)]
pub struct IChatSyncManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub AssociateAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    AssociateAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnassociateAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnassociateAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub IsAccountAssociated: unsafe extern "system" fn(this: *mut *mut Self, webaccount: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IsAccountAssociated: usize,
    pub StartSync: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut *mut Self, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetConfigurationAsync: usize,
}
impl ::windows_sys::core::Interface for IChatSyncManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2074422371, data2: 9808, data3: 18543, data4: [180, 180, 107, 217, 211, 214, 60, 132] };
}
#[repr(C)]
pub struct IRcsEndUserMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsPinRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, action: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseWithPinAsync: unsafe extern "system" fn(this: *mut *mut Self, action: *mut ::core::ffi::c_void, pin: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseWithPinAsync: usize,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3620578795, data2: 52183, data3: 20283, data4: [133, 38, 181, 6, 222, 195, 92, 83] };
}
#[repr(C)]
pub struct IRcsEndUserMessageAction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessageAction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2453112631, data2: 39746, data3: 18131, data4: [157, 94, 60, 27, 45, 174, 124, 184] };
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMessageAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessageAvailableEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 759541249, data2: 16265, data3: 16874, data4: [151, 2, 158, 158, 212, 17, 170, 152] };
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessageAvailableTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1536652333, data2: 13599, data3: 18066, data4: [180, 30, 27, 3, 93, 193, 137, 134] };
}
#[repr(C)]
pub struct IRcsEndUserMessageManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MessageAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageAvailableChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageAvailableChanged: usize,
}
impl ::windows_sys::core::Interface for IRcsEndUserMessageManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810856026, data2: 19743, data3: 19289, data4: [148, 51, 18, 108, 115, 78, 134, 166] };
}
#[repr(C)]
pub struct IRcsManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetEndUserMessageManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut *mut Self, transportid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveConversationAsync: unsafe extern "system" fn(this: *mut *mut Self, conversation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveConversationAsync: usize,
}
impl ::windows_sys::core::Interface for IRcsManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099710661, data2: 2749, data3: 20273, data4: [155, 153, 165, 158, 113, 167, 183, 49] };
}
#[repr(C)]
pub struct IRcsManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TransportListChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransportListChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransportListChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransportListChanged: usize,
}
impl ::windows_sys::core::Interface for IRcsManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3444157720, data2: 44426, data3: 17066, data4: [142, 235, 167, 152, 168, 128, 137, 89] };
}
#[repr(C)]
pub struct IRcsServiceKindSupportedChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RcsServiceKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRcsServiceKindSupportedChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4101939780, data2: 59267, data3: 18534, data4: [179, 167, 78, 92, 207, 2, 48, 112] };
}
#[repr(C)]
pub struct IRcsTransport {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsStoreAndForwardEnabled: unsafe extern "system" fn(this: *mut *mut Self, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsServiceKindSupported: unsafe extern "system" fn(this: *mut *mut Self, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceKindSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceKindSupportedChanged: usize,
}
impl ::windows_sys::core::Interface for IRcsTransport {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4272113497, data2: 62332, data3: 17177, data4: [133, 70, 236, 132, 210, 29, 48, 255] };
}
#[repr(C)]
pub struct IRcsTransportConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxGroupMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub MaxFileSizeInKilobytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub WarningFileSizeInKilobytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRcsTransportConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 533508354, data2: 9330, data3: 19385, data4: [153, 136, 193, 33, 28, 131, 232, 169] };
}
#[repr(C)]
pub struct IRemoteParticipantComposingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParticipantAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsComposing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteParticipantComposingChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 515917223, data2: 53193, data3: 17865, data4: [152, 118, 68, 159, 43, 193, 128, 245] };
}
pub type RcsEndUserMessage = *mut ::core::ffi::c_void;
pub type RcsEndUserMessageAction = *mut ::core::ffi::c_void;
pub type RcsEndUserMessageAvailableEventArgs = *mut ::core::ffi::c_void;
pub type RcsEndUserMessageAvailableTriggerDetails = *mut ::core::ffi::c_void;
pub type RcsEndUserMessageManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
impl ::core::marker::Copy for RcsServiceKind {}
impl ::core::clone::Clone for RcsServiceKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RcsServiceKindSupportedChangedEventArgs = *mut ::core::ffi::c_void;
pub type RcsTransport = *mut ::core::ffi::c_void;
pub type RcsTransportConfiguration = *mut ::core::ffi::c_void;
pub type RemoteParticipantComposingChangedEventArgs = *mut ::core::ffi::c_void;
