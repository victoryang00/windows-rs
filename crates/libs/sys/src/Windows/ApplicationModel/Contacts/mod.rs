#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
pub type AggregateContactManager = *mut ::core::ffi::c_void;
pub type Contact = *mut ::core::ffi::c_void;
pub type ContactAddress = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAddressKind(pub i32);
impl ContactAddressKind {
    pub const Home: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactAddressKind {}
impl ::core::clone::Clone for ContactAddressKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactAnnotation = *mut ::core::ffi::c_void;
pub type ContactAnnotationList = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationOperations(pub u32);
impl ContactAnnotationOperations {
    pub const None: Self = Self(0u32);
    pub const ContactProfile: Self = Self(1u32);
    pub const Message: Self = Self(2u32);
    pub const AudioCall: Self = Self(4u32);
    pub const VideoCall: Self = Self(8u32);
    pub const SocialFeeds: Self = Self(16u32);
    pub const Share: Self = Self(32u32);
}
impl ::core::marker::Copy for ContactAnnotationOperations {}
impl ::core::clone::Clone for ContactAnnotationOperations {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactAnnotationStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: Self = Self(0i32);
    pub const AllAnnotationsReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactAnnotationStoreAccessType {}
impl ::core::clone::Clone for ContactAnnotationStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactBatch = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactBatchStatus {}
impl ::core::clone::Clone for ContactBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactCardDelayedDataLoader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactCardHeaderKind {}
impl ::core::clone::Clone for ContactCardHeaderKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactCardOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactCardTabKind(pub i32);
impl ContactCardTabKind {
    pub const Default: Self = Self(0i32);
    pub const Email: Self = Self(1i32);
    pub const Messaging: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const OrganizationalHierarchy: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactCardTabKind {}
impl ::core::clone::Clone for ContactCardTabKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactChange = *mut ::core::ffi::c_void;
pub type ContactChangeReader = *mut ::core::ffi::c_void;
pub type ContactChangeTracker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactChangeType {}
impl ::core::clone::Clone for ContactChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactChangedDeferral = *mut ::core::ffi::c_void;
pub type ContactChangedEventArgs = *mut ::core::ffi::c_void;
pub type ContactConnectedServiceAccount = *mut ::core::ffi::c_void;
pub type ContactDate = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactDateKind(pub i32);
impl ContactDateKind {
    pub const Birthday: Self = Self(0i32);
    pub const Anniversary: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactDateKind {}
impl ::core::clone::Clone for ContactDateKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactEmail = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: Self = Self(0i32);
    pub const Work: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactEmailKind {}
impl ::core::clone::Clone for ContactEmailKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactField = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Mobile: Self = Self(3i32);
    pub const Other: Self = Self(4i32);
}
impl ::core::marker::Copy for ContactFieldCategory {}
impl ::core::clone::Clone for ContactFieldCategory {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactFieldFactory = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactFieldType(pub i32);
impl ContactFieldType {
    pub const Email: Self = Self(0i32);
    pub const PhoneNumber: Self = Self(1i32);
    pub const Location: Self = Self(2i32);
    pub const InstantMessage: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const ConnectedServiceAccount: Self = Self(5i32);
    pub const ImportantDate: Self = Self(6i32);
    pub const Address: Self = Self(7i32);
    pub const SignificantOther: Self = Self(8i32);
    pub const Notes: Self = Self(9i32);
    pub const Website: Self = Self(10i32);
    pub const JobInfo: Self = Self(11i32);
}
impl ::core::marker::Copy for ContactFieldType {}
impl ::core::clone::Clone for ContactFieldType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactGroup = *mut ::core::ffi::c_void;
pub type ContactInformation = *mut ::core::ffi::c_void;
pub type ContactInstantMessageField = *mut ::core::ffi::c_void;
pub type ContactJobInfo = *mut ::core::ffi::c_void;
pub type ContactList = *mut ::core::ffi::c_void;
pub type ContactListLimitedWriteOperations = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListOtherAppReadAccess(pub i32);
impl ContactListOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for ContactListOtherAppReadAccess {}
impl ::core::clone::Clone for ContactListOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactListOtherAppWriteAccess {}
impl ::core::clone::Clone for ContactListOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactListSyncConstraints = *mut ::core::ffi::c_void;
pub type ContactListSyncManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactListSyncStatus(pub i32);
impl ContactListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for ContactListSyncStatus {}
impl ::core::clone::Clone for ContactListSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactLocationField = *mut ::core::ffi::c_void;
pub type ContactManagerForUser = *mut ::core::ffi::c_void;
pub type ContactMatchReason = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactMatchReasonKind(pub i32);
impl ContactMatchReasonKind {
    pub const Name: Self = Self(0i32);
    pub const EmailAddress: Self = Self(1i32);
    pub const PhoneNumber: Self = Self(2i32);
    pub const JobInfo: Self = Self(3i32);
    pub const YomiName: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactMatchReasonKind {}
impl ::core::clone::Clone for ContactMatchReasonKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: Self = Self(0i32);
    pub const LastNameFirstName: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactNameOrder {}
impl ::core::clone::Clone for ContactNameOrder {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactPanel = *mut ::core::ffi::c_void;
pub type ContactPanelClosingEventArgs = *mut ::core::ffi::c_void;
pub type ContactPanelLaunchFullAppRequestedEventArgs = *mut ::core::ffi::c_void;
pub type ContactPhone = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactPhoneKind(pub i32);
impl ContactPhoneKind {
    pub const Home: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const Work: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const Pager: Self = Self(4i32);
    pub const BusinessFax: Self = Self(5i32);
    pub const HomeFax: Self = Self(6i32);
    pub const Company: Self = Self(7i32);
    pub const Assistant: Self = Self(8i32);
    pub const Radio: Self = Self(9i32);
}
impl ::core::marker::Copy for ContactPhoneKind {}
impl ::core::clone::Clone for ContactPhoneKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactPicker = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: Self = Self(0u32);
    pub const PhoneNumber: Self = Self(1u32);
    pub const EmailAddress: Self = Self(2u32);
    pub const PostalAddress: Self = Self(4u32);
}
impl ::core::marker::Copy for ContactQueryDesiredFields {}
impl ::core::clone::Clone for ContactQueryDesiredFields {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactQueryOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Name: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Phone: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for ContactQuerySearchFields {}
impl ::core::clone::Clone for ContactQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactQuerySearchScope {}
impl ::core::clone::Clone for ContactQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactQueryTextSearch = *mut ::core::ffi::c_void;
pub type ContactReader = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactRelationship(pub i32);
impl ContactRelationship {
    pub const Other: Self = Self(0i32);
    pub const Spouse: Self = Self(1i32);
    pub const Partner: Self = Self(2i32);
    pub const Sibling: Self = Self(3i32);
    pub const Parent: Self = Self(4i32);
    pub const Child: Self = Self(5i32);
}
impl ::core::marker::Copy for ContactRelationship {}
impl ::core::clone::Clone for ContactRelationship {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: Self = Self(0i32);
    pub const Fields: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactSelectionMode {}
impl ::core::clone::Clone for ContactSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactSignificantOther = *mut ::core::ffi::c_void;
pub type ContactStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: Self = Self(0i32);
    pub const AllContactsReadOnly: Self = Self(1i32);
    pub const AllContactsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactStoreAccessType {}
impl ::core::clone::Clone for ContactStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactStoreNotificationTriggerDetails = *mut ::core::ffi::c_void;
pub type ContactWebsite = *mut ::core::ffi::c_void;
pub type FullContactCardOptions = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAggregateContactManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindRawContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindRawContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryLinkContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, primarycontact: *mut ::core::ffi::c_void, secondarycontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryLinkContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnlinkRawContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnlinkRawContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetPreferredSourceForPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, aggregatecontact: *mut ::core::ffi::c_void, rawcontact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetPreferredSourceForPictureAsync: usize,
}
impl ::windows_sys::core::Interface for IAggregateContactManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 58316253, data2: 56154, data3: 20435, data4: [181, 78, 77, 241, 121, 23, 162, 18] };
}
#[repr(C)]
pub struct IAggregateContactManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SetRemoteIdentificationInformationAsync: unsafe extern "system" fn(this: *mut *mut Self, contactlistid: ::windows_sys::core::HSTRING, remotesourceid: ::windows_sys::core::HSTRING, accountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRemoteIdentificationInformationAsync: usize,
}
impl ::windows_sys::core::Interface for IAggregateContactManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1586283224, data2: 43469, data3: 17456, data4: [156, 75, 1, 52, 141, 178, 202, 80] };
}
#[repr(C)]
pub struct IContact {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Fields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Fields: usize,
}
impl ::windows_sys::core::Interface for IContact {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3959452403, data2: 8472, data3: 16457, data4: [158, 188, 23, 240, 171, 105, 43, 100] };
}
#[repr(C)]
pub struct IContact2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNotes: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Phones: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Phones: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Addresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Addresses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConnectedServiceAccounts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConnectedServiceAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportantDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportantDates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSuppliers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSuppliers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub JobInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    JobInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SignificantOthers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SignificantOthers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Websites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Websites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
impl ::windows_sys::core::Interface for IContact2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4078105445, data2: 47991, data3: 19604, data4: [128, 45, 131, 40, 206, 228, 12, 8] };
}
#[repr(C)]
pub struct IContact3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayPictureUserUpdateTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayPictureUserUpdateTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayPictureUserUpdateTime: usize,
    pub IsMe: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RingToneToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRingToneToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsDisplayPictureManuallySet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LargeDisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LargeDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SmallDisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmallDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SourceDisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SourceDisplayPicture: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSourceDisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSourceDisplayPicture: usize,
    pub TextToneToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTextToneToken: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsAggregate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayNameOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayNameOverride: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNickname: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SortName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContact3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1210064487, data2: 57486, data3: 17060, data4: [181, 97, 65, 208, 140, 169, 87, 93] };
}
#[repr(C)]
pub struct IContactAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub StreetAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocality: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactAddressKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactAddressKind) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2537149338, data2: 17102, data3: 18546, data4: [141, 112, 48, 99, 170, 88, 75, 112] };
}
#[repr(C)]
pub struct IContactAnnotation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AnnotationListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ContactId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SupportedOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactAnnotationOperations) -> ::windows_sys::core::HRESULT,
    pub SetSupportedOperations: unsafe extern "system" fn(this: *mut *mut Self, value: ContactAnnotationOperations) -> ::windows_sys::core::HRESULT,
    pub IsDisabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
}
impl ::windows_sys::core::Interface for IContactAnnotation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2183119599, data2: 32065, data3: 17570, data4: [132, 195, 96, 162, 129, 221, 123, 134] };
}
#[repr(C)]
pub struct IContactAnnotation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContactListId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetContactListId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactAnnotation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3063016691, data2: 19127, data3: 18975, data4: [153, 65, 12, 156, 243, 23, 27, 117] };
}
#[repr(C)]
pub struct IContactAnnotationList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ProviderPackageFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAnnotationAsync: unsafe extern "system" fn(this: *mut *mut Self, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationAsync: unsafe extern "system" fn(this: *mut *mut Self, annotationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsByRemoteIdAsync: unsafe extern "system" fn(this: *mut *mut Self, remoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAnnotationAsync: unsafe extern "system" fn(this: *mut *mut Self, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAnnotationAsync: usize,
}
impl ::windows_sys::core::Interface for IContactAnnotationList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2460255914, data2: 23688, data3: 17849, data4: [170, 208, 70, 24, 136, 230, 141, 138] };
}
#[repr(C)]
pub struct IContactAnnotationStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByEmailAsync: unsafe extern "system" fn(this: *mut *mut Self, emailaddress: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByEmailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactIdsByPhoneNumberAsync: unsafe extern "system" fn(this: *mut *mut Self, phonenumber: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactIdsByPhoneNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAnnotationAsync: unsafe extern "system" fn(this: *mut *mut Self, annotation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAnnotationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAnnotationListInAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAnnotationListInAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAnnotationListAsync: unsafe extern "system" fn(this: *mut *mut Self, annotationlistid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnnotationListAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationListsAsync: usize,
}
impl ::windows_sys::core::Interface for IContactAnnotationStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 598537386, data2: 31351, data3: 17789, data4: [130, 3, 152, 127, 75, 49, 175, 9] };
}
#[repr(C)]
pub struct IContactAnnotationStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAnnotationsForContactListAsync: unsafe extern "system" fn(this: *mut *mut Self, contactlistid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAnnotationsForContactListAsync: usize,
}
impl ::windows_sys::core::Interface for IContactAnnotationStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2128487421, data2: 25063, data3: 18791, data4: [142, 197, 189, 242, 128, 162, 64, 99] };
}
#[repr(C)]
pub struct IContactBatch {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Contacts: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Contacts: usize,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactBatchStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactBatch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 902928173, data2: 49102, data3: 18107, data4: [147, 248, 165, 176, 110, 197, 226, 1] };
}
#[repr(C)]
pub struct IContactCardDelayedDataLoader {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactCardDelayedDataLoader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3054172418, data2: 5446, data3: 17229, data4: [134, 156, 110, 53, 32, 118, 14, 243] };
}
#[repr(C)]
pub struct IContactCardOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub HeaderKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactCardHeaderKind) -> ::windows_sys::core::HRESULT,
    pub SetHeaderKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactCardHeaderKind) -> ::windows_sys::core::HRESULT,
    pub InitialTabKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactCardTabKind) -> ::windows_sys::core::HRESULT,
    pub SetInitialTabKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactCardTabKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactCardOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2349485950, data2: 27318, data3: 20287, data4: [190, 114, 129, 114, 54, 238, 234, 91] };
}
#[repr(C)]
pub struct IContactCardOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerSearchContactListIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerSearchContactListIds: usize,
}
impl ::windows_sys::core::Interface for IContactCardOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2401704864, data2: 55115, data3: 19654, data4: [159, 83, 27, 14, 181, 209, 39, 60] };
}
#[repr(C)]
pub struct IContactChange {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactChangeType) -> ::windows_sys::core::HRESULT,
    pub Contact: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2501724944, data2: 27225, data3: 18208, data4: [164, 225, 54, 61, 152, 193, 53, 213] };
}
#[repr(C)]
pub struct IContactChangeReader {
    pub base__: ::windows_sys::core::IInspectable,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut *mut Self, lastchangetoaccept: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
impl ::windows_sys::core::Interface for IContactChangeReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 561191418, data2: 11532, data3: 17120, data4: [169, 218, 62, 205, 86, 167, 138, 71] };
}
#[repr(C)]
pub struct IContactChangeTracker {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChangeTracker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1855531346, data2: 12443, data3: 16461, data4: [151, 18, 179, 123, 211, 2, 120, 170] };
}
#[repr(C)]
pub struct IContactChangeTracker2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTracking: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChangeTracker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2139803900, data2: 37665, data3: 19736, data4: [156, 9, 215, 8, 198, 63, 205, 49] };
}
#[repr(C)]
pub struct IContactChangedDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChangedDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3306437352, data2: 6915, data3: 18168, data4: [182, 148, 165, 35, 232, 60, 252, 182] };
}
#[repr(C)]
pub struct IContactChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1381924817, data2: 29683, data3: 19325, data4: [169, 24, 88, 11, 228, 54, 97, 33] };
}
#[repr(C)]
pub struct IContactConnectedServiceAccount {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactConnectedServiceAccount {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4143461715, data2: 43559, data3: 18225, data4: [142, 74, 61, 236, 92, 233, 238, 201] };
}
#[repr(C)]
pub struct IContactDate {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Day: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub SetDay: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDay: usize,
    #[cfg(feature = "Foundation")]
    pub Month: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Month: usize,
    #[cfg(feature = "Foundation")]
    pub SetMonth: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Year: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Year: usize,
    #[cfg(feature = "Foundation")]
    pub SetYear: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetYear: usize,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactDateKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactDateKind) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactDate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4271418982, data2: 45573, data3: 18740, data4: [145, 116, 15, 242, 176, 86, 87, 7] };
}
#[repr(C)]
pub struct IContactEmail {
    pub base__: ::windows_sys::core::IInspectable,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactEmailKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactEmailKind) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactEmail {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2426542505, data2: 58323, data3: 19811, data4: [153, 59, 5, 185, 165, 57, 58, 191] };
}
#[repr(C)]
pub struct IContactField {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactFieldType) -> ::windows_sys::core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactFieldCategory) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactField {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2977319018, data2: 53907, data3: 18732, data4: [160, 88, 219, 87, 91, 62, 60, 15] };
}
#[repr(C)]
pub struct IContactFieldFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateField_Default: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateField_Category: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateField_Custom: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactFieldFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2246218047, data2: 3658, data3: 19006, data4: [137, 148, 64, 106, 231, 237, 100, 110] };
}
#[repr(C)]
pub struct IContactGroup {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IContactGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1505618689, data2: 40602, data3: 18269, data4: [191, 229, 163, 123, 128, 109, 133, 44] };
}
#[repr(C)]
pub struct IContactInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Emails: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Emails: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneNumbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstantMessages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstantMessages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomFields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomFields: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryCustomFields: unsafe extern "system" fn(this: *mut *mut Self, customname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryCustomFields: usize,
}
impl ::windows_sys::core::Interface for IContactInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 660518612, data2: 27182, data3: 17016, data4: [169, 20, 228, 96, 213, 240, 136, 246] };
}
#[repr(C)]
pub struct IContactInstantMessageField {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Service: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUri: usize,
}
impl ::windows_sys::core::Interface for IContactInstantMessageField {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3437443895, data2: 3461, data3: 16890, data4: [180, 61, 218, 89, 156, 62, 176, 9] };
}
#[repr(C)]
pub struct IContactInstantMessageFieldFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstantMessage_Default: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstantMessage_Category: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateInstantMessage_All: unsafe extern "system" fn(this: *mut *mut Self, username: ::windows_sys::core::HSTRING, category: ContactFieldCategory, service: ::windows_sys::core::HSTRING, displaytext: ::windows_sys::core::HSTRING, verb: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstantMessage_All: usize,
}
impl ::windows_sys::core::Interface for IContactInstantMessageFieldFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121309588, data2: 37283, data3: 19378, data4: [177, 185, 105, 165, 223, 240, 186, 9] };
}
#[repr(C)]
pub struct IContactJobInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub CompanyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompanyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CompanyYomiName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompanyYomiName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetManager: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Office: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetOffice: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CompanyAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCompanyAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactJobInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1829862220, data2: 52816, data3: 19267, data4: [158, 105, 177, 130, 88, 234, 83, 21] };
}
#[repr(C)]
pub struct IContactLaunchActionVerbsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Call: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Map: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Post: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub VideoCall: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactLaunchActionVerbsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4212273878, data2: 61043, data3: 18151, data4: [135, 97, 17, 205, 1, 87, 114, 143] };
}
#[repr(C)]
pub struct IContactList {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactListOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut *mut Self, value: ContactListOtherAppReadAccess) -> ::windows_sys::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactListOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut *mut Self, value: ContactListOtherAppWriteAccess) -> ::windows_sys::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SupportsServerSearch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactFromRemoteIdAsync: unsafe extern "system" fn(this: *mut *mut Self, remoteid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactFromRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contactid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
impl ::windows_sys::core::Interface for IContactList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 383642741, data2: 14636, data3: 18501, data4: [157, 251, 81, 163, 231, 239, 62, 66] };
}
#[repr(C)]
pub struct IContactList2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
    pub SetSupportsServerSearch: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub SyncConstraints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactList2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409527732, data2: 17744, data3: 19915, data4: [146, 41, 64, 255, 145, 251, 2, 3] };
}
#[repr(C)]
pub struct IContactList3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactList3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 360246871, data2: 9980, data3: 16872, data4: [168, 80, 90, 163, 37, 20, 172, 169] };
}
#[repr(C)]
pub struct IContactListLimitedWriteOperations {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contactid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteContactAsync: usize,
}
impl ::windows_sys::core::Interface for IContactListLimitedWriteOperations {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3784840154, data2: 18955, data3: 17592, data4: [154, 31, 160, 243, 210, 24, 23, 95] };
}
#[repr(C)]
pub struct IContactListSyncConstraints {
    pub base__: ::windows_sys::core::IInspectable,
    pub CanSyncDescriptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanSyncDescriptions: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomePhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxMobilePhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxMobilePhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPagerPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPagerPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBusinessFaxPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBusinessFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeFaxPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeFaxPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxCompanyPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxCompanyPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAssistantPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAssistantPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxRadioPhoneNumbers: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxRadioPhoneNumbers: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPersonalEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPersonalEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherEmailAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherEmailAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxHomeAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxHomeAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxHomeAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWorkAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWorkAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWorkAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherAddresses: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherAddresses: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherAddresses: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBirthdayDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxBirthdayDates: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxBirthdayDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxAnniversaryDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAnniversaryDates: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAnniversaryDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherDates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherDates: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherDates: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOtherRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxOtherRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxOtherRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSpouseRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSpouseRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSpouseRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxPartnerRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPartnerRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPartnerRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSiblingRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSiblingRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSiblingRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxParentRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxParentRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxParentRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxChildRelationships: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxChildRelationships: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxChildRelationships: usize,
    #[cfg(feature = "Foundation")]
    pub MaxJobInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxJobInfo: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxJobInfo: usize,
    #[cfg(feature = "Foundation")]
    pub MaxWebsites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxWebsites: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxWebsites: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxWebsites: usize,
}
impl ::windows_sys::core::Interface for IContactListSyncConstraints {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2997927681, data2: 12386, data3: 20014, data4: [150, 157, 1, 141, 25, 135, 243, 20] };
}
#[repr(C)]
pub struct IContactListSyncManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactListSyncStatus) -> ::windows_sys::core::HRESULT,
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
impl ::windows_sys::core::Interface for IContactListSyncManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342787006, data2: 31013, data3: 19148, data4: [157, 229, 33, 221, 208, 111, 134, 116] };
}
#[repr(C)]
pub struct IContactListSyncManager2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetStatus: unsafe extern "system" fn(this: *mut *mut Self, value: ContactListSyncStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
impl ::windows_sys::core::Interface for IContactListSyncManager2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2841186887, data2: 47957, data3: 20003, data4: [129, 40, 55, 1, 52, 168, 93, 13] };
}
#[repr(C)]
pub struct IContactLocationField {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnstructuredAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactLocationField {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2663387010, data2: 43886, data3: 19254, data4: [137, 227, 178, 59, 192, 161, 218, 204] };
}
#[repr(C)]
pub struct IContactLocationFieldFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateLocation_Default: unsafe extern "system" fn(this: *mut *mut Self, unstructuredaddress: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLocation_Category: unsafe extern "system" fn(this: *mut *mut Self, unstructuredaddress: ::windows_sys::core::HSTRING, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLocation_All: unsafe extern "system" fn(this: *mut *mut Self, unstructuredaddress: ::windows_sys::core::HSTRING, category: ContactFieldCategory, street: ::windows_sys::core::HSTRING, city: ::windows_sys::core::HSTRING, region: ::windows_sys::core::HSTRING, country: ::windows_sys::core::HSTRING, postalcode: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactLocationFieldFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4154012375, data2: 12255, data3: 17406, data4: [143, 24, 65, 137, 115, 144, 188, 254] };
}
#[repr(C)]
pub struct IContactManagerForUser {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut *mut Self, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ContactNameOrder) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IContactManagerForUser {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3075193431, data2: 4214, data3: 19439, data4: [174, 243, 84, 104, 109, 24, 56, 125] };
}
#[repr(C)]
pub struct IContactManagerForUser2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactManagerForUser2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1296473134, data2: 15221, data3: 19059, data4: [187, 48, 115, 102, 69, 71, 34, 86] };
}
#[repr(C)]
pub struct IContactManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ShowContactCard: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowContactCard: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithPlacement: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithPlacement: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCard: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCard: usize,
}
impl ::windows_sys::core::Interface for IContactManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2180127424, data2: 63073, data3: 18184, data4: [186, 79, 211, 134, 189, 13, 98, 46] };
}
#[repr(C)]
pub struct IContactManagerStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IContactManagerStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2709055008, data2: 18392, data3: 18636, data4: [150, 60, 149, 146, 182, 229, 16, 198] };
}
#[repr(C)]
pub struct IContactManagerStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertContactToVCardAsyncWithMaxBytes: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, maxbytes: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertContactToVCardAsyncWithMaxBytes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ConvertVCardToContactAsync: unsafe extern "system" fn(this: *mut *mut Self, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ConvertVCardToContactAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsyncWithAccessType: unsafe extern "system" fn(this: *mut *mut Self, accesstype: ContactStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsyncWithAccessType: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAnnotationStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, accesstype: ContactAnnotationStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAnnotationStoreAsync: usize,
    pub IsShowContactCardSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowContactCardWithOptions: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowContactCardWithOptions: usize,
    pub IsShowDelayLoadedContactCardSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowDelayLoadedContactCardWithOptions: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowDelayLoadedContactCardWithOptions: usize,
    pub ShowFullContactCard: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, fullcontactcardoptions: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SystemDisplayNameOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SetSystemDisplayNameOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SystemSortOrder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactNameOrder) -> ::windows_sys::core::HRESULT,
    pub SetSystemSortOrder: unsafe extern "system" fn(this: *mut *mut Self, value: ContactNameOrder) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactManagerStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3301719362, data2: 30086, data3: 18730, data4: [147, 11, 123, 193, 56, 252, 33, 57] };
}
#[repr(C)]
pub struct IContactManagerStatics4 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
impl ::windows_sys::core::Interface for IContactManagerStatics4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 613950066, data2: 13435, data3: 18140, data4: [141, 149, 81, 189, 65, 225, 90, 175] };
}
#[repr(C)]
pub struct IContactManagerStatics5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsShowFullContactCardSupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShowFullContactCardSupportedAsync: usize,
    pub IncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeMiddleNameInSystemDisplayAndSort: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactManagerStatics5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4149811847, data2: 44215, data3: 20397, data4: [144, 242, 168, 171, 100, 205, 187, 164] };
}
#[repr(C)]
pub struct IContactMatchReason {
    pub base__: ::windows_sys::core::IInspectable,
    pub Field: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactMatchReasonKind) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub Segments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    Segments: usize,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactMatchReason {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3163694340, data2: 59352, data3: 16702, data4: [149, 244, 183, 92, 84, 199, 64, 119] };
}
#[repr(C)]
pub struct IContactName {
    pub base__: ::windows_sys::core::IInspectable,
    pub FirstName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MiddleName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetMiddleName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiGivenName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetYomiGivenName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetYomiFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificNameSuffix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHonorificNameSuffix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHonorificNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactName {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4093962619, data2: 36916, data3: 17724, data4: [142, 191, 20, 10, 56, 200, 111, 29] };
}
#[repr(C)]
pub struct IContactPanel {
    pub base__: ::windows_sys::core::IInspectable,
    pub ClosePanel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub HeaderColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    HeaderColor: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub SetHeaderColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    SetHeaderColor: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFullAppRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLaunchFullAppRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLaunchFullAppRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Closing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosing: usize,
}
impl ::windows_sys::core::Interface for IContactPanel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1103041125, data2: 53998, data3: 19351, data4: [168, 10, 125, 141, 100, 204, 166, 245] };
}
#[repr(C)]
pub struct IContactPanelClosingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
impl ::windows_sys::core::Interface for IContactPanelClosingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 572617939, data2: 53067, data3: 18135, data4: [183, 57, 110, 220, 22, 17, 11, 251] };
}
#[repr(C)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactPanelLaunchFullAppRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2295733262, data2: 9140, data3: 19432, data4: [138, 252, 7, 44, 37, 164, 25, 13] };
}
#[repr(C)]
pub struct IContactPhone {
    pub base__: ::windows_sys::core::IInspectable,
    pub Number: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactPhoneKind) -> ::windows_sys::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut *mut Self, value: ContactPhoneKind) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactPhone {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1182640997, data2: 10002, data3: 20306, data4: [183, 131, 158, 168, 17, 28, 99, 205] };
}
#[repr(C)]
pub struct IContactPicker {
    pub base__: ::windows_sys::core::IInspectable,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SelectionMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactSelectionMode) -> ::windows_sys::core::HRESULT,
    pub SetSelectionMode: unsafe extern "system" fn(this: *mut *mut Self, value: ContactSelectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFields: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleContactAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleContactsAsync: usize,
}
impl ::windows_sys::core::Interface for IContactPicker {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 235535761, data2: 17144, data3: 16469, data4: [144, 160, 137, 111, 150, 115, 137, 54] };
}
#[repr(C)]
pub struct IContactPicker2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
    #[cfg(feature = "Foundation")]
    pub PickContactAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickContactsAsync: usize,
}
impl ::windows_sys::core::Interface for IContactPicker2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3008369103, data2: 23791, data3: 19748, data4: [170, 12, 52, 12, 82, 8, 114, 93] };
}
#[repr(C)]
pub struct IContactPicker3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
impl ::windows_sys::core::Interface for IContactPicker3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242365205, data2: 45635, data3: 19437, data4: [133, 22, 34, 177, 167, 172, 10, 206] };
}
#[repr(C)]
pub struct IContactPickerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
}
impl ::windows_sys::core::Interface for IContactPickerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1955119145, data2: 27219, data3: 16984, data4: [163, 233, 98, 223, 246, 120, 75, 108] };
}
#[repr(C)]
pub struct IContactQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextSearch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactListIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactListIds: usize,
    pub IncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeContactsFromHiddenLists: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub DesiredFields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactQueryDesiredFields) -> ::windows_sys::core::HRESULT,
    pub SetDesiredFields: unsafe extern "system" fn(this: *mut *mut Self, value: ContactQueryDesiredFields) -> ::windows_sys::core::HRESULT,
    pub DesiredOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactAnnotationOperations) -> ::windows_sys::core::HRESULT,
    pub SetDesiredOperations: unsafe extern "system" fn(this: *mut *mut Self, value: ContactAnnotationOperations) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AnnotationListIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AnnotationListIds: usize,
}
impl ::windows_sys::core::Interface for IContactQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1141427358, data2: 32124, data3: 17136, data4: [138, 199, 245, 7, 51, 236, 219, 193] };
}
#[repr(C)]
pub struct IContactQueryOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithText: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut *mut Self, text: ::windows_sys::core::HSTRING, fields: ContactQuerySearchFields, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactQueryOptionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1413462599, data2: 36071, data3: 18123, data4: [157, 172, 154, 164, 42, 27, 200, 226] };
}
#[repr(C)]
pub struct IContactQueryTextSearch {
    pub base__: ::windows_sys::core::IInspectable,
    pub Fields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactQuerySearchFields) -> ::windows_sys::core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut *mut Self, value: ContactQuerySearchFields) -> ::windows_sys::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactQuerySearchScope) -> ::windows_sys::core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut *mut Self, value: ContactQuerySearchScope) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactQueryTextSearch {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4158912971, data2: 43351, data3: 17307, data4: [160, 183, 28, 2, 161, 150, 63, 240] };
}
#[repr(C)]
pub struct IContactReader {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMatchingPropertiesWithMatchReason: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMatchingPropertiesWithMatchReason: usize,
}
impl ::windows_sys::core::Interface for IContactReader {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3549946926, data2: 5256, data3: 17138, data4: [191, 100, 37, 63, 72, 132, 191, 237] };
}
#[repr(C)]
pub struct IContactSignificantOther {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactSignificantOther {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2289284523, data2: 50683, data3: 18136, data4: [147, 254, 218, 63, 241, 147, 64, 84] };
}
#[repr(C)]
pub struct IContactSignificantOther2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Relationship: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactRelationship) -> ::windows_sys::core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(this: *mut *mut Self, value: ContactRelationship) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactSignificantOther2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2373702772, data2: 16131, data3: 17912, data4: [186, 15, 196, 237, 55, 214, 66, 25] };
}
#[repr(C)]
pub struct IContactStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactsWithSearchTextAsync: unsafe extern "system" fn(this: *mut *mut Self, searchtext: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactsWithSearchTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contactid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactAsync: usize,
}
impl ::windows_sys::core::Interface for IContactStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 740428560, data2: 14956, data3: 17043, data4: [185, 188, 254, 152, 127, 110, 13, 82] };
}
#[repr(C)]
pub struct IContactStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactChanged: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactChanged: usize,
    pub AggregateContactManager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindContactListsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetContactListAsync: unsafe extern "system" fn(this: *mut *mut Self, contactlistid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateContactListAsync: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMeContactAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMeContactAsync: usize,
    pub GetContactReader: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetContactReaderWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateContactListInAccountAsync: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, userdataaccountid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateContactListInAccountAsync: usize,
}
impl ::windows_sys::core::Interface for IContactStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 416160802, data2: 60373, data3: 19451, data4: [182, 144, 95, 79, 39, 196, 240, 232] };
}
#[repr(C)]
pub struct IContactStore3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut *mut Self, identity: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactStore3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3414699116, data2: 78, data3: 16464, data4: [135, 240, 132, 4, 7, 238, 104, 24] };
}
#[repr(C)]
pub struct IContactStoreNotificationTriggerDetails {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IContactStoreNotificationTriggerDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2880608470, data2: 34698, data3: 20363, data4: [169, 206, 70, 187, 125, 28, 132, 206] };
}
#[repr(C)]
pub struct IContactWebsite {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactWebsite {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2668822902, data2: 56347, data3: 16469, data4: [173, 102, 101, 47, 57, 217, 144, 232] };
}
#[repr(C)]
pub struct IContactWebsite2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RawValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRawValue: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactWebsite2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169066782, data2: 22087, data3: 16488, data4: [187, 94, 75, 111, 67, 124, 227, 8] };
}
#[repr(C)]
pub struct IFullContactCardOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::ViewManagement::ViewSizePreference) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
impl ::windows_sys::core::Interface for IFullContactCardOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2269397868, data2: 23801, data3: 18051, data4: [189, 202, 161, 253, 235, 248, 219, 206] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IKnownContactFieldStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Email: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Email: usize,
    #[cfg(feature = "deprecated")]
    pub PhoneNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PhoneNumber: usize,
    #[cfg(feature = "deprecated")]
    pub Location: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Location: usize,
    #[cfg(feature = "deprecated")]
    pub InstantMessage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InstantMessage: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertNameToType: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut ContactFieldType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertNameToType: usize,
    #[cfg(feature = "deprecated")]
    pub ConvertTypeToName: unsafe extern "system" fn(this: *mut *mut Self, r#type: ContactFieldType, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConvertTypeToName: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IKnownContactFieldStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 772676370, data2: 54823, data3: 20426, data4: [186, 212, 31, 175, 22, 140, 125, 20] };
}
#[repr(C)]
pub struct IPinnedContactIdsQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub ContactIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContactIds: usize,
}
impl ::windows_sys::core::Interface for IPinnedContactIdsQueryResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2107319634, data2: 5497, data3: 19932, data4: [135, 31, 163, 10, 58, 234, 155, 161] };
}
#[repr(C)]
pub struct IPinnedContactManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub IsPinSurfaceSupported: unsafe extern "system" fn(this: *mut *mut Self, surface: PinnedContactSurface, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsContactPinned: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestPinContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestPinContactAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestPinContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, contacts: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestPinContactsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestUnpinContactAsync: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void, surface: PinnedContactSurface, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnpinContactAsync: usize,
    pub SignalContactActivity: unsafe extern "system" fn(this: *mut *mut Self, contact: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPinnedContactIdsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPinnedContactIdsAsync: usize,
}
impl ::windows_sys::core::Interface for IPinnedContactManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4240208908, data2: 57814, data3: 17859, data4: [184, 182, 163, 86, 4, 225, 103, 160] };
}
#[repr(C)]
pub struct IPinnedContactManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPinnedContactManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4133276798, data2: 65017, data3: 18538, data4: [172, 233, 188, 49, 29, 10, 231, 240] };
}
pub type PinnedContactIdsQueryResult = *mut ::core::ffi::c_void;
pub type PinnedContactManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
#[repr(transparent)]
pub struct PinnedContactSurface(pub i32);
impl PinnedContactSurface {
    pub const StartMenu: Self = Self(0i32);
    pub const Taskbar: Self = Self(1i32);
}
impl ::core::marker::Copy for PinnedContactSurface {}
impl ::core::clone::Clone for PinnedContactSurface {
    fn clone(&self) -> Self {
        *self
    }
}
