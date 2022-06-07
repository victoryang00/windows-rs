#[cfg(feature = "Phone_PersonalInformation_Provisioning")]
pub mod Provisioning;
pub type ContactAddress = *mut ::core::ffi::c_void;
pub type ContactChangeRecord = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: Self = Self(0i32);
    pub const Modified: Self = Self(1i32);
    pub const Deleted: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactChangeType {}
impl ::core::clone::Clone for ContactChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactInformation = *mut ::core::ffi::c_void;
pub type ContactQueryOptions = *mut ::core::ffi::c_void;
pub type ContactQueryResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactQueryResultOrdering(pub i32);
impl ContactQueryResultOrdering {
    pub const SystemDefault: Self = Self(0i32);
    pub const GivenNameFamilyName: Self = Self(1i32);
    pub const FamilyNameGivenName: Self = Self(2i32);
}
impl ::core::marker::Copy for ContactQueryResultOrdering {}
impl ::core::clone::Clone for ContactQueryResultOrdering {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ContactStore = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactStoreApplicationAccessMode(pub i32);
impl ContactStoreApplicationAccessMode {
    pub const LimitedReadOnly: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreApplicationAccessMode {}
impl ::core::clone::Clone for ContactStoreApplicationAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct ContactStoreSystemAccessMode(pub i32);
impl ContactStoreSystemAccessMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for ContactStoreSystemAccessMode {}
impl ::core::clone::Clone for ContactStoreSystemAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IContactAddress {
    pub base__: ::windows_sys::core::IInspectable,
    pub Country: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetCountry: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetLocality: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRegion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPostalCode: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StreetAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactAddress {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1596258599, data2: 38057, data3: 17570, data4: [161, 85, 45, 11, 55, 209, 220, 205] };
}
#[repr(C)]
pub struct IContactChangeRecord {
    pub base__: ::windows_sys::core::IInspectable,
    pub ChangeType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactChangeType) -> ::windows_sys::core::HRESULT,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactChangeRecord {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3117676431, data2: 20795, data3: 18242, data4: [190, 0, 204, 92, 92, 35, 107, 4] };
}
#[repr(C)]
pub struct IContactInformation {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetFamilyName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GivenName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetGivenName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificPrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHonorificPrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificSuffix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetHonorificSuffix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDisplayPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDisplayPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetDisplayPictureAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetDisplayPictureAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DisplayPicture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DisplayPicture: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ToVcardAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ToVcardAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ToVcardWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, format: VCardFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ToVcardWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IContactInformation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3803521020, data2: 59282, data3: 19127, data4: [177, 91, 242, 224, 120, 102, 77, 234] };
}
#[repr(C)]
pub struct IContactInformation2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub DisplayPictureDate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayPictureDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDisplayPictureDate: unsafe extern "system" fn(this: *mut *mut Self, returnvalue: super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDisplayPictureDate: usize,
}
impl ::windows_sys::core::Interface for IContactInformation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 832090636, data2: 25118, data3: 18024, data4: [172, 56, 214, 103, 184, 125, 6, 213] };
}
#[repr(C)]
pub struct IContactInformationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ParseVcardAsync: unsafe extern "system" fn(this: *mut *mut Self, vcard: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ParseVcardAsync: usize,
}
impl ::windows_sys::core::Interface for IContactInformationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 258456361, data2: 976, data3: 19430, data4: [178, 165, 251, 19, 133, 159, 18, 2] };
}
#[repr(C)]
pub struct IContactQueryOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFields: usize,
    pub OrderBy: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ContactQueryResultOrdering) -> ::windows_sys::core::HRESULT,
    pub SetOrderBy: unsafe extern "system" fn(this: *mut *mut Self, value: ContactQueryResultOrdering) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactQueryOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1477225334, data2: 16177, data3: 18113, data4: [154, 80, 66, 74, 83, 218, 202, 227] };
}
#[repr(C)]
pub struct IContactQueryResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetContactCountAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetContactCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContactsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContactsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContactsAsyncInRange: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, maxnumberofitems: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContactsAsyncInRange: usize,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IContactQueryResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3225270050, data2: 60635, data3: 18176, data4: [133, 126, 62, 120, 100, 38, 176, 75] };
}
#[repr(C)]
pub struct IContactStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FindContactByRemoteIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindContactByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FindContactByIdAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindContactByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteContactAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteContactAsync: usize,
    pub CreateContactQueryDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateContactQueryWithOptions: unsafe extern "system" fn(this: *mut *mut Self, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetChangesAsync: unsafe extern "system" fn(this: *mut *mut Self, baserevisionnumber: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetChangesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadExtendedPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SaveExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SaveExtendedPropertiesAsync: usize,
}
impl ::windows_sys::core::Interface for IContactStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2999807983, data2: 11261, data3: 20397, data4: [133, 82, 78, 105, 128, 151, 232, 235] };
}
#[repr(C)]
pub struct IContactStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateMeContactAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMeContactAsync: usize,
}
impl ::windows_sys::core::Interface for IContactStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1710339663, data2: 54867, data3: 17319, data4: [178, 54, 179, 12, 15, 77, 114, 105] };
}
#[repr(C)]
pub struct IContactStoreStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateOrOpenAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrOpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateOrOpenWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, access: ContactStoreSystemAccessMode, sharing: ContactStoreApplicationAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrOpenWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IContactStoreStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2818899490, data2: 19435, data3: 17612, data4: [165, 114, 103, 165, 185, 46, 133, 103] };
}
#[repr(C)]
pub struct IKnownContactPropertiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GivenName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificPrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HonorificSuffix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AdditionalName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OtherAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Email: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WorkAddress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WorkTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Birthdate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Anniversary: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Telephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MobileTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Url: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Notes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WorkFax: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SignificantOther: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CompanyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub CompanyTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HomeFax: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlternateTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OfficeLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub WorkEmail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiGivenName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub YomiCompanyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub OtherEmail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlternateMobileTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AlternateWorkTelephone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownContactPropertiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3582012161, data2: 11501, data3: 20198, data4: [177, 214, 9, 75, 248, 142, 240, 182] };
}
#[repr(C)]
pub struct IStoredContact {
    pub base__: ::windows_sys::core::IInspectable,
    pub Store: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtendedPropertiesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtendedPropertiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReplaceExistingContactAsync: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReplaceExistingContactAsync: usize,
}
impl ::windows_sys::core::Interface for IStoredContact {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2960177073, data2: 9789, data3: 20081, data4: [171, 231, 89, 29, 36, 102, 87, 14] };
}
#[repr(C)]
pub struct IStoredContactFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateStoredContact: unsafe extern "system" fn(this: *mut *mut Self, store: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStoredContactFromInformation: unsafe extern "system" fn(this: *mut *mut Self, store: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStoredContactFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1240328481, data2: 49701, data3: 20441, data4: [137, 197, 206, 204, 44, 138, 75, 121] };
}
pub type StoredContact = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Phone_PersonalInformation\"`*"]
#[repr(transparent)]
pub struct VCardFormat(pub i32);
impl VCardFormat {
    pub const Version2_1: Self = Self(0i32);
    pub const Version3: Self = Self(1i32);
}
impl ::core::marker::Copy for VCardFormat {}
impl ::core::clone::Clone for VCardFormat {
    fn clone(&self) -> Self {
        *self
    }
}
