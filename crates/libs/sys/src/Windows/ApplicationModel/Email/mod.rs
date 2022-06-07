#[cfg(feature = "ApplicationModel_Email_DataProvider")]
pub mod DataProvider;
pub type EmailAttachment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailAttachmentDownloadState(pub i32);
impl EmailAttachmentDownloadState {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailAttachmentDownloadState {}
impl ::core::clone::Clone for EmailAttachmentDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailBatchStatus(pub i32);
impl EmailBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailBatchStatus {}
impl ::core::clone::Clone for EmailBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailCertificateValidationStatus(pub i32);
impl EmailCertificateValidationStatus {
    pub const Success: Self = Self(0i32);
    pub const NoMatch: Self = Self(1i32);
    pub const InvalidUsage: Self = Self(2i32);
    pub const InvalidCertificate: Self = Self(3i32);
    pub const Revoked: Self = Self(4i32);
    pub const ChainRevoked: Self = Self(5i32);
    pub const RevocationServerFailure: Self = Self(6i32);
    pub const Expired: Self = Self(7i32);
    pub const Untrusted: Self = Self(8i32);
    pub const ServerError: Self = Self(9i32);
    pub const UnknownFailure: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailCertificateValidationStatus {}
impl ::core::clone::Clone for EmailCertificateValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailConversation = *mut ::core::ffi::c_void;
pub type EmailConversationBatch = *mut ::core::ffi::c_void;
pub type EmailConversationReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailFlagState(pub i32);
impl EmailFlagState {
    pub const Unflagged: Self = Self(0i32);
    pub const Flagged: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Cleared: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailFlagState {}
impl ::core::clone::Clone for EmailFlagState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailFolder = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailImportance(pub i32);
impl EmailImportance {
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailImportance {}
impl ::core::clone::Clone for EmailImportance {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailIrmInfo = *mut ::core::ffi::c_void;
pub type EmailIrmTemplate = *mut ::core::ffi::c_void;
pub type EmailItemCounts = *mut ::core::ffi::c_void;
pub type EmailMailbox = *mut ::core::ffi::c_void;
pub type EmailMailboxAction = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxActionKind(pub i32);
impl EmailMailboxActionKind {
    pub const MarkMessageAsSeen: Self = Self(0i32);
    pub const MarkMessageRead: Self = Self(1i32);
    pub const ChangeMessageFlagState: Self = Self(2i32);
    pub const MoveMessage: Self = Self(3i32);
    pub const SaveDraft: Self = Self(4i32);
    pub const SendMessage: Self = Self(5i32);
    pub const CreateResponseReplyMessage: Self = Self(6i32);
    pub const CreateResponseReplyAllMessage: Self = Self(7i32);
    pub const CreateResponseForwardMessage: Self = Self(8i32);
    pub const MoveFolder: Self = Self(9i32);
    pub const MarkFolderForSyncEnabled: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailMailboxActionKind {}
impl ::core::clone::Clone for EmailMailboxActionKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(pub i32);
impl EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    pub const None: Self = Self(0i32);
    pub const StrongAlgorithm: Self = Self(1i32);
    pub const AnyAlgorithm: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {}
impl ::core::clone::Clone for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMailboxAutoReply = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxAutoReplyMessageResponseKind(pub i32);
impl EmailMailboxAutoReplyMessageResponseKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxAutoReplyMessageResponseKind {}
impl ::core::clone::Clone for EmailMailboxAutoReplyMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMailboxAutoReplySettings = *mut ::core::ffi::c_void;
pub type EmailMailboxCapabilities = *mut ::core::ffi::c_void;
pub type EmailMailboxChange = *mut ::core::ffi::c_void;
pub type EmailMailboxChangeReader = *mut ::core::ffi::c_void;
pub type EmailMailboxChangeTracker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChangeType(pub i32);
impl EmailMailboxChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const FolderCreated: Self = Self(3i32);
    pub const FolderModified: Self = Self(4i32);
    pub const FolderDeleted: Self = Self(5i32);
    pub const ChangeTrackingLost: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxChangeType {}
impl ::core::clone::Clone for EmailMailboxChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMailboxChangedDeferral = *mut ::core::ffi::c_void;
pub type EmailMailboxChangedEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxCreateFolderResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxCreateFolderStatus(pub i32);
impl EmailMailboxCreateFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const NameCollision: Self = Self(5i32);
    pub const ServerRejected: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxCreateFolderStatus {}
impl ::core::clone::Clone for EmailMailboxCreateFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderStatus(pub i32);
impl EmailMailboxDeleteFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxDeleteFolderStatus {}
impl ::core::clone::Clone for EmailMailboxDeleteFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderStatus(pub i32);
impl EmailMailboxEmptyFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxEmptyFolderStatus {}
impl ::core::clone::Clone for EmailMailboxEmptyFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxOtherAppReadAccess(pub i32);
impl EmailMailboxOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppReadAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxOtherAppWriteAccess(pub i32);
impl EmailMailboxOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppWriteAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMailboxPolicies = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(pub i32);
impl EmailMailboxSmimeEncryptionAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const TripleDes: Self = Self(1i32);
    pub const Des: Self = Self(2i32);
    pub const RC2128Bit: Self = Self(3i32);
    pub const RC264Bit: Self = Self(4i32);
    pub const RC240Bit: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeEncryptionAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSmimeSigningAlgorithm(pub i32);
impl EmailMailboxSmimeSigningAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const Sha1: Self = Self(1i32);
    pub const MD5: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeSigningAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeSigningAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMailboxSyncManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSyncStatus(pub i32);
impl EmailMailboxSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxSyncStatus {}
impl ::core::clone::Clone for EmailMailboxSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailManagerForUser = *mut ::core::ffi::c_void;
pub type EmailMeetingInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMeetingResponseType(pub i32);
impl EmailMeetingResponseType {
    pub const Accept: Self = Self(0i32);
    pub const Decline: Self = Self(1i32);
    pub const Tentative: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMeetingResponseType {}
impl ::core::clone::Clone for EmailMeetingResponseType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMessage = *mut ::core::ffi::c_void;
pub type EmailMessageBatch = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageBodyKind(pub i32);
impl EmailMessageBodyKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMessageBodyKind {}
impl ::core::clone::Clone for EmailMessageBodyKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageDownloadState(pub i32);
impl EmailMessageDownloadState {
    pub const PartiallyDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageDownloadState {}
impl ::core::clone::Clone for EmailMessageDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailMessageReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageResponseKind(pub i32);
impl EmailMessageResponseKind {
    pub const None: Self = Self(0i32);
    pub const Reply: Self = Self(1i32);
    pub const ReplyAll: Self = Self(2i32);
    pub const Forward: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageResponseKind {}
impl ::core::clone::Clone for EmailMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageSmimeKind(pub i32);
impl EmailMessageSmimeKind {
    pub const None: Self = Self(0i32);
    pub const ClearSigned: Self = Self(1i32);
    pub const OpaqueSigned: Self = Self(2i32);
    pub const Encrypted: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageSmimeKind {}
impl ::core::clone::Clone for EmailMessageSmimeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQueryKind(pub i32);
impl EmailQueryKind {
    pub const All: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Flagged: Self = Self(2i32);
    pub const Unread: Self = Self(3i32);
    pub const Read: Self = Self(4i32);
    pub const Unseen: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailQueryKind {}
impl ::core::clone::Clone for EmailQueryKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailQueryOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQuerySearchFields(pub u32);
impl EmailQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Subject: Self = Self(1u32);
    pub const Sender: Self = Self(2u32);
    pub const Preview: Self = Self(4u32);
    pub const Recipients: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for EmailQuerySearchFields {}
impl ::core::clone::Clone for EmailQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQuerySearchScope(pub i32);
impl EmailQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySearchScope {}
impl ::core::clone::Clone for EmailQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQuerySortDirection(pub i32);
impl EmailQuerySortDirection {
    pub const Descending: Self = Self(0i32);
    pub const Ascending: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySortDirection {}
impl ::core::clone::Clone for EmailQuerySortDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQuerySortProperty(pub i32);
impl EmailQuerySortProperty {
    pub const Date: Self = Self(0i32);
}
impl ::core::marker::Copy for EmailQuerySortProperty {}
impl ::core::clone::Clone for EmailQuerySortProperty {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailQueryTextSearch = *mut ::core::ffi::c_void;
pub type EmailRecipient = *mut ::core::ffi::c_void;
pub type EmailRecipientResolutionResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailRecipientResolutionStatus(pub i32);
impl EmailRecipientResolutionStatus {
    pub const Success: Self = Self(0i32);
    pub const RecipientNotFound: Self = Self(1i32);
    pub const AmbiguousRecipient: Self = Self(2i32);
    pub const NoCertificate: Self = Self(3i32);
    pub const CertificateRequestLimitReached: Self = Self(4i32);
    pub const CannotResolveDistributionList: Self = Self(5i32);
    pub const ServerError: Self = Self(6i32);
    pub const UnknownFailure: Self = Self(7i32);
}
impl ::core::marker::Copy for EmailRecipientResolutionStatus {}
impl ::core::clone::Clone for EmailRecipientResolutionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailSpecialFolderKind(pub i32);
impl EmailSpecialFolderKind {
    pub const None: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const Inbox: Self = Self(2i32);
    pub const Outbox: Self = Self(3i32);
    pub const Drafts: Self = Self(4i32);
    pub const DeletedItems: Self = Self(5i32);
    pub const Sent: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailSpecialFolderKind {}
impl ::core::clone::Clone for EmailSpecialFolderKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailStoreAccessType(pub i32);
impl EmailStoreAccessType {
    pub const AppMailboxesReadWrite: Self = Self(0i32);
    pub const AllMailboxesLimitedReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailStoreAccessType {}
impl ::core::clone::Clone for EmailStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EmailStoreNotificationTriggerDetails = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEmailAttachment {
    pub base__: ::windows_sys::core::IInspectable,
    pub FileName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
impl ::windows_sys::core::Interface for IEmailAttachment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4082354937, data2: 22472, data3: 19163, data4: [185, 146, 96, 252, 235, 88, 79, 84] };
}
#[repr(C)]
pub struct IEmailAttachment2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContentLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContentLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailAttachmentDownloadState) -> ::windows_sys::core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut *mut Self, value: EmailAttachmentDownloadState) -> ::windows_sys::core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    pub IsFromBaseMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsInline: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInline: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailAttachment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 576655472, data2: 45311, data3: 17777, data4: [157, 84, 167, 6, 196, 141, 85, 198] };
}
#[repr(C)]
pub struct IEmailAttachmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IEmailAttachmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2037296198, data2: 60758, data3: 18809, data4: [135, 8, 171, 184, 188, 133, 75, 125] };
}
#[repr(C)]
pub struct IEmailAttachmentFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, filename: ::windows_sys::core::HSTRING, data: *mut ::core::ffi::c_void, mimetype: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IEmailAttachmentFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 589665333, data2: 20985, data3: 17021, data4: [173, 205, 36, 16, 35, 200, 207, 183] };
}
#[repr(C)]
pub struct IEmailConversation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailFlagState) -> ::windows_sys::core::HRESULT,
    pub HasAttachment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailImportance) -> ::windows_sys::core::HRESULT,
    pub LastEmailResponseKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMessageResponseKind) -> ::windows_sys::core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MostRecentMessageTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MostRecentMessageTime: usize,
    pub Preview: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LatestSender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UnreadMessageCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesWithCountAsync: unsafe extern "system" fn(this: *mut *mut Self, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesWithCountAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailConversation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659055688, data2: 41148, data3: 17225, data4: [144, 45, 144, 246, 99, 137, 245, 27] };
}
#[repr(C)]
pub struct IEmailConversationBatch {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Conversations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Conversations: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailBatchStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailConversationBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3099700097, data2: 453, data3: 17194, data4: [157, 241, 254, 133, 217, 138, 39, 154] };
}
#[repr(C)]
pub struct IEmailConversationReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailConversationReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3026390914, data2: 10357, data3: 17608, data4: [155, 140, 133, 190, 179, 163, 198, 83] };
}
#[repr(C)]
pub struct IEmailFolder {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailSpecialFolderKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindChildFoldersAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindChildFoldersAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageCountsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageCountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveAsync: unsafe extern "system" fn(this: *mut *mut Self, newparentfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveWithNewNameAsync: unsafe extern "system" fn(this: *mut *mut Self, newparentfolder: *mut ::core::ffi::c_void, newfoldername: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailFolder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2723116913, data2: 39276, data3: 18532, data4: [177, 186, 237, 18, 64, 229, 125, 17] };
}
#[repr(C)]
pub struct IEmailIrmInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanEdit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanEdit: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanExtractData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanExtractData: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanForward: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanForward: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanPrintData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanPrintData: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanReply: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanReply: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub CanReplyAll: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanReplyAll: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationDate: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationDate: usize,
    pub IsIrmOriginator: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsIrmOriginator: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailIrmInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2431984019, data2: 45472, data3: 20157, data4: [166, 182, 221, 202, 85, 96, 110, 14] };
}
#[repr(C)]
pub struct IEmailIrmInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, expiration: super::super::Foundation::DateTime, irmtemplate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IEmailIrmInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 827044236, data2: 58342, data3: 19835, data4: [190, 141, 145, 169, 99, 17, 176, 27] };
}
#[repr(C)]
pub struct IEmailIrmTemplate {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailIrmTemplate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4079449485, data2: 21613, data3: 19434, data4: [169, 99, 84, 163, 139, 44, 192, 22] };
}
#[repr(C)]
pub struct IEmailIrmTemplateFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, description: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailIrmTemplateFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1034098806, data2: 34616, data3: 17432, data4: [185, 203, 71, 27, 147, 111, 231, 30] };
}
#[repr(C)]
pub struct IEmailItemCounts {
    pub base__: ::windows_sys::core::IInspectable,
    pub Flagged: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Important: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Total: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Unread: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailItemCounts {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1540436769, data2: 65224, data3: 19371, data4: [131, 186, 11, 175, 60, 31, 108, 189] };
}
#[repr(C)]
pub struct IEmailMailbox {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsOwnedByCurrentApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsDataEncryptedUnderLock: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MailAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMailAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailAddressAliases: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailAddressAliases: usize,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMailboxOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMailboxOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub Policies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSpecialFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, foldertype: EmailSpecialFolderKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSpecialFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderAsSeenAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, isread: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeMessageFlagStateAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, flagstate: EmailFlagState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeMessageFlagStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, newparentfolderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, newparentfolderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderWithNewNameAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, newparentfolderid: ::windows_sys::core::HSTRING, newfoldername: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderSyncEnabledAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, issyncenabled: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderSyncEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveDraftAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveDraftAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadAttachmentAsync: unsafe extern "system" fn(this: *mut *mut Self, attachmentid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadAttachmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateResponseMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, messageid: ::windows_sys::core::HSTRING, responsetype: EmailMessageResponseKind, subject: ::windows_sys::core::HSTRING, responseheadertype: EmailMessageBodyKind, responseheader: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateResponseMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, response: EmailMeetingResponseType, subject: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, sendupdate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, recipients: *mut ::core::ffi::c_void, subject: ::windows_sys::core::HSTRING, forwardheadertype: EmailMessageBodyKind, forwardheader: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut *mut Self, meeting: *mut ::core::ffi::c_void, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::windows_sys::core::HSTRING, comment: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MailboxChanged: unsafe extern "system" fn(this: *mut *mut Self, phandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMailboxChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SmartSendMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, smartsend: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SmartSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, autoreplysettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetAutoReplySettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut *mut Self, requestedformat: EmailMailboxAutoReplyMessageResponseKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAutoReplySettingsAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailbox {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826503753, data2: 53083, data3: 16667, data4: [128, 177, 74, 106, 20, 132, 206, 37] };
}
#[repr(C)]
pub struct IEmailMailbox2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LinkedMailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NetworkId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailbox2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 351855620, data2: 27810, data3: 19122, data4: [146, 65, 121, 205, 123, 244, 99, 70] };
}
#[repr(C)]
pub struct IEmailMailbox3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveRecipientsAsync: unsafe extern "system" fn(this: *mut *mut Self, recipients: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveRecipientsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ValidateCertificatesAsync: unsafe extern "system" fn(this: *mut *mut Self, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ValidateCertificatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEmptyFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEmptyFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, parentfolderid: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, folderid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteFolderAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailbox3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1034258811, data2: 17803, data3: 16522, data4: [142, 55, 172, 139, 5, 216, 175, 86] };
}
#[repr(C)]
pub struct IEmailMailbox4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailbox4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1562325019, data2: 61986, data3: 18599, data4: [183, 182, 113, 99, 86, 205, 38, 161] };
}
#[repr(C)]
pub struct IEmailMailbox5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailbox5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 966160519, data2: 146, data3: 18878, data4: [189, 14, 93, 77, 201, 217, 109, 144] };
}
#[repr(C)]
pub struct IEmailMailboxAction {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxActionKind) -> ::windows_sys::core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxAction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2895677946, data2: 8698, data3: 18727, data4: [146, 16, 212, 16, 88, 47, 223, 62] };
}
#[repr(C)]
pub struct IEmailMailboxAutoReply {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxAutoReply {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3793954124, data2: 35508, data3: 18523, data4: [179, 31, 4, 209, 84, 118, 189, 89] };
}
#[repr(C)]
pub struct IEmailMailboxAutoReplySettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ResponseKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxAutoReplyMessageResponseKind) -> ::windows_sys::core::HRESULT,
    pub SetResponseKind: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    pub InternalReply: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub KnownExternalReply: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UnknownExternalReply: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxAutoReplySettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2826608552, data2: 2758, data3: 19319, data4: [186, 119, 166, 185, 158, 154, 39, 184] };
}
#[repr(C)]
pub struct IEmailMailboxCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanServerSearchFolders: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanServerSearchMailbox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanSmartSend: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxCapabilities {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4007576486, data2: 35291, data3: 17157, data4: [130, 196, 67, 158, 10, 51, 218, 17] };
}
#[repr(C)]
pub struct IEmailMailboxCapabilities2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanResolveRecipients: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanValidateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanEmptyFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanCreateFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanDeleteFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CanMoveFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxCapabilities2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1769094884, data2: 12065, data3: 19644, data4: [136, 171, 46, 118, 2, 164, 128, 107] };
}
#[repr(C)]
pub struct IEmailMailboxCapabilities3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanServerSearchFolders: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanServerSearchMailbox: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanSmartSend: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanResolveRecipients: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanValidateCertificates: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanEmptyFolder: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanCreateFolder: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanDeleteFolder: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetCanMoveFolder: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxCapabilities3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4136692036, data2: 22258, data3: 17834, data4: [135, 44, 12, 233, 243, 219, 11, 92] };
}
#[repr(C)]
pub struct IEmailMailboxChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxChangeType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailboxActions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailboxActions: usize,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1642984779, data2: 4591, data3: 16396, data4: [173, 222, 140, 222, 101, 200, 94, 102] };
}
#[repr(C)]
pub struct IEmailMailboxChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut *mut Self, lastchangetoacknowledge: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3183283899, data2: 50493, data3: 17201, data4: [151, 190, 190, 117, 162, 20, 106, 117] };
}
#[repr(C)]
pub struct IEmailMailboxChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTracking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxChangeTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061796920, data2: 20838, data3: 17079, data4: [136, 130, 253, 33, 201, 43, 221, 75] };
}
#[repr(C)]
pub struct IEmailMailboxChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxChangedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2006611137, data2: 38853, data3: 19284, data4: [179, 13, 48, 98, 50, 98, 62, 109] };
}
#[repr(C)]
pub struct IEmailMailboxChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1023237998, data2: 468, data3: 20042, data4: [164, 76, 178, 45, 212, 46, 194, 7] };
}
#[repr(C)]
pub struct IEmailMailboxCreateFolderResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxCreateFolderStatus) -> ::windows_sys::core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxCreateFolderResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2988987775, data2: 10373, data3: 18840, data4: [181, 149, 138, 45, 55, 76, 233, 80] };
}
#[repr(C)]
pub struct IEmailMailboxPolicies {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows_sys::core::HRESULT,
    pub AllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeSigningAlgorithm: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxPolicies {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 523453893, data2: 7227, data3: 19911, data4: [180, 16, 99, 115, 120, 62, 84, 93] };
}
#[repr(C)]
pub struct IEmailMailboxPolicies2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub MustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MustSignSmimeMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxPolicies2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3132459771, data2: 41291, data3: 18812, data4: [168, 226, 85, 234, 194, 156, 196, 181] };
}
#[repr(C)]
pub struct IEmailMailboxPolicies3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetAllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows_sys::core::HRESULT,
    pub SetAllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeSigningAlgorithm: usize,
    pub SetMustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SetMustSignSmimeMessages: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMailboxPolicies3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3184828447, data2: 18535, data3: 16714, data4: [129, 162, 128, 57, 25, 196, 65, 145] };
}
#[repr(C)]
pub struct IEmailMailboxSyncManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMailboxSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSyncManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1367000410, data2: 13713, data3: 19293, data4: [133, 188, 199, 29, 222, 134, 34, 99] };
}
#[repr(C)]
pub struct IEmailMailboxSyncManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMailboxSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
impl ::windows_sys::core::Interface for IEmailMailboxSyncManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3448621438, data2: 38337, data3: 20361, data4: [129, 183, 230, 174, 203, 102, 149, 252] };
}
#[repr(C)]
pub struct IEmailManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: EmailStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IEmailManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4151565983, data2: 15525, data3: 19215, data4: [144, 193, 21, 110, 64, 23, 76, 229] };
}
#[repr(C)]
pub struct IEmailManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut *mut Self, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4111631956, data2: 21957, data3: 18576, data4: [168, 36, 33, 108, 38, 24, 206, 127] };
}
#[repr(C)]
pub struct IEmailManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: EmailStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2886020515, data2: 45460, data3: 16989, data4: [182, 217, 208, 240, 65, 53, 237, 162] };
}
#[repr(C)]
pub struct IEmailManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IEmailManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1248994197, data2: 33854, data3: 18757, data4: [179, 170, 52, 158, 7, 163, 98, 197] };
}
#[repr(C)]
pub struct IEmailMeetingInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AppointmentRoamingId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAppointmentRoamingId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsAllDay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAllDay: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProposedStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedStartTime: unsafe extern "system" fn(this: *mut *mut Self, proposedstarttime: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ProposedDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedDuration: unsafe extern "system" fn(this: *mut *mut Self, duration: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub RecurrenceStartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecurrenceStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecurrenceStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecurrenceStartTime: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Recurrence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Recurrence: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub SetRecurrence: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    SetRecurrence: usize,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, value: u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
impl ::windows_sys::core::Interface for IEmailMeetingInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 834682793, data2: 31027, data3: 16735, data4: [162, 117, 209, 101, 186, 7, 2, 107] };
}
#[repr(C)]
pub struct IEmailMeetingInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReportedOutOfDateByServer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMeetingInfo2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2119776365, data2: 45273, data3: 20453, data4: [134, 124, 227, 30, 210, 181, 136, 184] };
}
#[repr(C)]
pub struct IEmailMessage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Subject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub To: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    To: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CC: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CC: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Bcc: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bcc: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
}
impl ::windows_sys::core::Interface for IEmailMessage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1819120781, data2: 32949, data3: 18680, data4: [176, 177, 224, 78, 67, 15, 68, 229] };
}
#[repr(C)]
pub struct IEmailMessage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FolderId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AllowInternetImages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAllowInternetImages: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMessageDownloadState) -> ::windows_sys::core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMessageDownloadState) -> ::windows_sys::core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailFlagState) -> ::windows_sys::core::HRESULT,
    pub SetFlagState: unsafe extern "system" fn(this: *mut *mut Self, value: EmailFlagState) -> ::windows_sys::core::HRESULT,
    pub HasPartialBodies: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailImportance) -> ::windows_sys::core::HRESULT,
    pub SetImportance: unsafe extern "system" fn(this: *mut *mut Self, value: EmailImportance) -> ::windows_sys::core::HRESULT,
    pub InResponseToMessageId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IrmInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetIrmInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsDraftMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsServerSearchMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSmartSendable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMessageClass: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub NormalizedSubject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OriginalCodePage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetOriginalCodePage: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Preview: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LastResponseKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMessageResponseKind) -> ::windows_sys::core::HRESULT,
    pub SetLastResponseKind: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMessageResponseKind) -> ::windows_sys::core::HRESULT,
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSender: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SentTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SentTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSentTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSentTime: usize,
    pub MeetingInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMeetingInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBodyStream: unsafe extern "system" fn(this: *mut *mut Self, r#type: EmailMessageBodyKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBodyStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBodyStream: unsafe extern "system" fn(this: *mut *mut Self, r#type: EmailMessageBodyKind, stream: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBodyStream: usize,
}
impl ::windows_sys::core::Interface for IEmailMessage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4257752203, data2: 40730, data3: 17627, data4: [189, 60, 101, 195, 132, 119, 15, 134] };
}
#[repr(C)]
pub struct IEmailMessage3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub SmimeData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmimeData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSmimeData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSmimeData: usize,
    pub SmimeKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailMessageSmimeKind) -> ::windows_sys::core::HRESULT,
    pub SetSmimeKind: unsafe extern "system" fn(this: *mut *mut Self, value: EmailMessageSmimeKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMessage3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2716493660, data2: 58776, data3: 19753, data4: [160, 24, 252, 123, 126, 236, 224, 161] };
}
#[repr(C)]
pub struct IEmailMessage4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ReplyTo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReplyTo: usize,
    pub SentRepresenting: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSentRepresenting: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMessage4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 830271873, data2: 15999, data3: 18949, data4: [131, 148, 62, 16, 51, 109, 212, 53] };
}
#[repr(C)]
pub struct IEmailMessageBatch {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Messages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Messages: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailBatchStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailMessageBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1616695439, data2: 9689, data3: 20251, data4: [158, 81, 5, 20, 192, 20, 150, 83] };
}
#[repr(C)]
pub struct IEmailMessageReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailMessageReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 793427615, data2: 25107, data3: 19077, data4: [163, 176, 249, 45, 26, 131, 157, 25] };
}
#[repr(C)]
pub struct IEmailQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextSearch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SortDirection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailQuerySortDirection) -> ::windows_sys::core::HRESULT,
    pub SetSortDirection: unsafe extern "system" fn(this: *mut *mut Self, value: EmailQuerySortDirection) -> ::windows_sys::core::HRESULT,
    pub SortProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailQuerySortProperty) -> ::windows_sys::core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut *mut Self, value: EmailQuerySortProperty) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailQueryKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: EmailQueryKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FolderIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FolderIds: usize,
}
impl ::windows_sys::core::Interface for IEmailQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1162890139, data2: 15999, data3: 19794, data4: [182, 221, 214, 253, 78, 31, 189, 154] };
}
#[repr(C)]
pub struct IEmailQueryOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithText: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, fields: EmailQuerySearchFields, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailQueryOptionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297536952, data2: 30891, data3: 20200, data4: [180, 227, 4, 109, 110, 47, 229, 226] };
}
#[repr(C)]
pub struct IEmailQueryTextSearch {
    pub base__: ::windows_sys::core::IInspectable,
    pub Fields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailQuerySearchFields) -> ::windows_sys::core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut *mut Self, value: EmailQuerySearchFields) -> ::windows_sys::core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailQuerySearchScope) -> ::windows_sys::core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut *mut Self, value: EmailQuerySearchScope) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailQueryTextSearch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2678104712, data2: 15453, data3: 18085, data4: [166, 226, 49, 214, 253, 23, 229, 64] };
}
#[repr(C)]
pub struct IEmailRecipient {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailRecipient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3404211635, data2: 17528, data3: 18452, data4: [185, 0, 201, 2, 181, 225, 155, 83] };
}
#[repr(C)]
pub struct IEmailRecipientFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut *mut Self, address: ::windows_sys::core::HSTRING, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IEmailRecipientFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1426110541, data2: 51098, data3: 20216, data4: [185, 9, 114, 46, 24, 227, 147, 93] };
}
#[repr(C)]
pub struct IEmailRecipientResolutionResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut EmailRecipientResolutionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub PublicKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    PublicKeys: usize,
}
impl ::windows_sys::core::Interface for IEmailRecipientResolutionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2441296122, data2: 36237, data3: 17779, data4: [128, 209, 7, 23, 42, 52, 185, 141] };
}
#[repr(C)]
pub struct IEmailRecipientResolutionResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: EmailRecipientResolutionStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub SetPublicKeys: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    SetPublicKeys: usize,
}
impl ::windows_sys::core::Interface for IEmailRecipientResolutionResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1581386678, data2: 52827, data3: 19422, data4: [185, 212, 225, 109, 160, 176, 159, 202] };
}
#[repr(C)]
pub struct IEmailStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMailboxesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMailboxesAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMailboxAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxAsync: unsafe extern "system" fn(this: *mut *mut Self, accountname: ::windows_sys::core::HSTRING, accountaddress: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxInAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, accountname: ::windows_sys::core::HSTRING, accountaddress: ::windows_sys::core::HSTRING, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxInAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IEmailStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4160954990, data2: 37175, data3: 20363, data4: [164, 112, 39, 154, 195, 5, 142, 182] };
}
#[repr(C)]
pub struct IEmailStoreNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IEmailStoreNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3457635900, data2: 18150, data3: 17353, data4: [150, 247, 250, 207, 125, 215, 16, 203] };
}
