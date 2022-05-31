#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountPartnerAccountInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AccountKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderAddAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UserDataAccountContentKinds) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PartnerAccountInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PartnerAccountInfos: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IUserDataAccountProviderOperation(::windows_core::IUnknown);
impl IUserDataAccountProviderOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountProviderOperationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows_core::IUnknown {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows_core::IUnknown {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows_core::IInspectable {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows_core::IInspectable {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserDataAccountProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserDataAccountProviderOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderOperation {}
impl ::core::fmt::Debug for IUserDataAccountProviderOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDataAccountProviderOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderResolveErrorsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderSettingsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UserDataAccountPartnerAccountInfo(::windows_core::IUnknown);
impl UserDataAccountPartnerAccountInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn AccountKind(&self) -> ::windows_core::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountProviderPartnerAccountKind>::zeroed();
            (::windows_core::Interface::vtable(this).AccountKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountProviderPartnerAccountKind>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountPartnerAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountPartnerAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountPartnerAccountInfo {}
impl ::core::fmt::Debug for UserDataAccountPartnerAccountInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountPartnerAccountInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountPartnerAccountInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::core::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[repr(transparent)]
pub struct UserDataAccountProviderAddAccountOperation(::windows_core::IUnknown);
impl UserDataAccountProviderAddAccountOperation {
    pub fn ContentKinds(&self) -> ::windows_core::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::UserDataAccountContentKinds>::zeroed();
            (::windows_core::Interface::vtable(this).ContentKinds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataAccountContentKinds>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PartnerAccountInfos)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>(result__)
        }
    }
    pub fn ReportCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), userdataaccountid.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountProviderOperationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderAddAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderAddAccountOperation {}
impl ::core::fmt::Debug for UserDataAccountProviderAddAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderAddAccountOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountProviderAddAccountOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: UserDataAccountProviderAddAccountOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserDataAccountProviderAddAccountOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderAddAccountOperation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataAccountProviderOperationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataAccountProviderOperationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountProviderOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderOperationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataAccountProviderPartnerAccountKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserDataAccountProviderPartnerAccountKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountProviderPartnerAccountKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderPartnerAccountKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(::windows_core::IUnknown);
impl UserDataAccountProviderResolveErrorsOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountProviderOperationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderResolveErrorsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderResolveErrorsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderResolveErrorsOperation {}
impl ::core::fmt::Debug for UserDataAccountProviderResolveErrorsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderResolveErrorsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountProviderResolveErrorsOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: UserDataAccountProviderResolveErrorsOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserDataAccountProviderResolveErrorsOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderResolveErrorsOperation {}
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(::windows_core::IUnknown);
impl UserDataAccountProviderSettingsOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserDataAccountProviderOperationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderSettingsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderSettingsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderSettingsOperation {}
impl ::core::fmt::Debug for UserDataAccountProviderSettingsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderSettingsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountProviderSettingsOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: UserDataAccountProviderSettingsOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserDataAccountProviderSettingsOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderSettingsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderSettingsOperation {}
