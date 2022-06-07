#[repr(C)]
pub struct IUserDataAccountPartnerAccountInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AccountKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccountPartnerAccountInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1595932727, data2: 63215, data3: 20163, data4: [134, 48, 1, 44, 89, 193, 20, 159] };
}
#[repr(C)]
pub struct IUserDataAccountProviderAddAccountOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub ContentKinds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::UserDataAccountContentKinds) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PartnerAccountInfos: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PartnerAccountInfos: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self, userdataaccountid: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccountProviderAddAccountOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3116836144, data2: 16260, data3: 19293, data4: [142, 170, 69, 233, 122, 168, 66, 237] };
}
#[repr(C)]
pub struct IUserDataAccountProviderOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut UserDataAccountProviderOperationKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccountProviderOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2718608739, data2: 34956, data3: 19042, data4: [163, 221, 52, 208, 122, 128, 43, 43] };
}
#[repr(C)]
pub struct IUserDataAccountProviderResolveErrorsOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccountProviderResolveErrorsOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1647696917, data2: 49099, data3: 16865, data4: [153, 87, 151, 89, 162, 136, 70, 204] };
}
#[repr(C)]
pub struct IUserDataAccountProviderSettingsOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUserDataAccountProviderSettingsOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2449690039, data2: 34376, data3: 20272, data4: [172, 250, 48, 2, 101, 140, 168, 13] };
}
pub type UserDataAccountPartnerAccountInfo = *mut ::core::ffi::c_void;
pub type UserDataAccountProviderAddAccountOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: Self = Self(0i32);
    pub const Settings: Self = Self(1i32);
    pub const ResolveErrors: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAccountProviderOperationKind {}
impl ::core::clone::Clone for UserDataAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: Self = Self(0i32);
    pub const PopOrImap: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataAccountProviderPartnerAccountKind {}
impl ::core::clone::Clone for UserDataAccountProviderPartnerAccountKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type UserDataAccountProviderResolveErrorsOperation = *mut ::core::ffi::c_void;
pub type UserDataAccountProviderSettingsOperation = *mut ::core::ffi::c_void;
