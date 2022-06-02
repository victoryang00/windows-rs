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
#[repr(C)]
pub struct IChatConversation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanModifyParticipants: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanModifyParticipants: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub ItemKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatItemKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatMessage3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessage4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SyncId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSyncId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatMessageAttachmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub CreateChatMessageAttachment: unsafe extern "system" fn(this: *mut *mut Self, mimetype: ::windows_sys::core::HSTRING, datastreamreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateChatMessageAttachment: usize,
}
#[repr(C)]
pub struct IChatMessageBlockingStatic {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsBlockedAsync: unsafe extern "system" fn(this: *mut *mut Self, localchatmessageid: ::windows_sys::core::HSTRING, blocked: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsBlockedAsync: usize,
}
#[repr(C)]
pub struct IChatMessageChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageChangeType) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatMessageChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessageChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessageChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatMessageManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSyncManagerAsync: usize,
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChatMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessageNotificationTriggerDetails2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShouldDisplayToast: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateDetailText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateBadge: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ShouldUpdateActionCenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IChatMessageReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[repr(C)]
pub struct IChatMessageReader2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut *mut Self, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
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
#[repr(C)]
pub struct IChatMessageStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetMessageBySyncIdAsync: unsafe extern "system" fn(this: *mut *mut Self, syncid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageBySyncIdAsync: usize,
}
#[repr(C)]
pub struct IChatMessageStoreChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatStoreChangedEventKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatMessageTransport2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Configuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TransportKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatMessageTransportKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub SearchString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSearchString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IChatSyncConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RestoreHistorySpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ChatRestoreHistorySpan) -> ::windows_sys::core::HRESULT,
    pub SetRestoreHistorySpan: unsafe extern "system" fn(this: *mut *mut Self, value: ChatRestoreHistorySpan) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IRcsEndUserMessageAction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Label: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMessageAvailable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IRcsServiceKindSupportedChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ServiceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RcsServiceKind) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IRemoteParticipantComposingChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub TransportId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParticipantAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsComposing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
