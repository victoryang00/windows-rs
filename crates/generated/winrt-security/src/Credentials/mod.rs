#[cfg(feature = "UI")]
pub mod UI;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICredentialFactory {
    type Vtable = ICredentialFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54ef13a1_bf26_47b5_97dd_de779b7cad58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreatePasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredential(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredential {
    type Vtable = IKeyCredential_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9585ef8d_457b_4847_b11a_fa960bbdb138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredential_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RetrievePublicKeyWithDefaultBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RetrievePublicKeyWithDefaultBlobType: usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub RetrievePublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))]
    RetrievePublicKeyWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RequestSignAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RequestSignAsync: usize,
    pub GetAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78aab3a1_a3c1_4103_b6cc_472c44171cbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CertificateChainBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CertificateChainBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationBuffer: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialManagerStatics {
    type Vtable = IKeyCredentialManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aac468b_0ef1_4ce0_8290_4106da6a63b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RenewAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf53786c1_5261_4cdd_976d_cc909ac71620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58cd7703_8d87_4249_9b58_f6598cc9644e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Credential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordCredential(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ab18989_c720_41a7_a6c1_feadb36329a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordCredential_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RetrievePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordVault(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPasswordVault {
    type Vtable = IPasswordVault_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61fd2c0b_c8d4_48c1_a54f_bc5a64205af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordVault_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByResource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByUserName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrieveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrieveAll: usize,
}
#[repr(transparent)]
pub struct IWebAccount(::windows_core::IUnknown);
impl IWebAccount {
    pub fn WebAccountProvider(&self) -> ::windows_core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebAccountProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebAccountState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountState>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccount> for ::windows_core::IUnknown {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows_core::IUnknown {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccount> for ::windows_core::IInspectable {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows_core::IInspectable {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccount {}
impl ::core::fmt::Debug for IWebAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebAccount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{69473eb2-8031-49be-80bb-96cb46d99aba}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebAccount {
    type Vtable = IWebAccount_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69473eb2_8031_49be_80bb_96cb46d99aba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccount2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccount2 {
    type Vtable = IWebAccount2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b56d6f8_990b_4eb5_94a7_5621f3a8b824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPictureAsync: usize,
    pub SignOutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SignOutWithClientIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountFactory {
    type Vtable = IWebAccountFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac9afb39_1de9_4e92_b78f_0581a87f6e5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows_core::RawPtr, username: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, state: WebAccountState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29dcc8c3_7ab9_4a7c_a336_b942f9dbf7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IconUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider2 {
    type Vtable = IWebAccountProvider2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a01eb05_4e42_41d4_b518_e008a5163614);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayPurpose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider3 {
    type Vtable = IWebAccountProvider3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda1c518b_970d_4d49_825c_f2706f8ca7fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider4 {
    type Vtable = IWebAccountProvider4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x718fd8db_e796_4210_b74e_84d29894b080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSystemProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderFactory {
    type Vtable = IWebAccountProviderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d767df1_e1e1_4b9a_a774_5c7c7e3bf371);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, iconuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct KeyCredential(::windows_core::IUnknown);
impl KeyCredential {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePublicKeyWithDefaultBlobType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePublicKeyWithBlobType)(::windows_core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RequestSignAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<KeyCredentialOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestSignAsync)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<KeyCredentialOperationResult>>(result__)
        }
    }
    pub fn GetAttestationAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<KeyCredentialAttestationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttestationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<KeyCredentialAttestationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredential {}
impl ::core::fmt::Debug for KeyCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredential").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredential {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredential;{9585ef8d-457b-4847-b11a-fa960bbdb138})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyCredential {
    type Vtable = IKeyCredential_Vtbl;
    const IID: ::windows_core::GUID = <IKeyCredential as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredential";
}
impl ::core::convert::From<KeyCredential> for ::windows_core::IUnknown {
    fn from(value: KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredential> for ::windows_core::IUnknown {
    fn from(value: &KeyCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyCredential> for ::windows_core::IInspectable {
    fn from(value: KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredential> for ::windows_core::IInspectable {
    fn from(value: &KeyCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredential {}
unsafe impl ::core::marker::Sync for KeyCredential {}
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(::windows_core::IUnknown);
impl KeyCredentialAttestationResult {
    #[cfg(feature = "Storage_Streams")]
    pub fn CertificateChainBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CertificateChainBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationBuffer(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttestationBuffer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyCredentialAttestationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialAttestationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialAttestationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialAttestationResult {}
impl ::core::fmt::Debug for KeyCredentialAttestationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialAttestationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialAttestationResult;{78aab3a1-a3c1-4103-b6cc-472c44171cbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
    const IID: ::windows_core::GUID = <IKeyCredentialAttestationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialAttestationResult";
}
impl ::core::convert::From<KeyCredentialAttestationResult> for ::windows_core::IUnknown {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for ::windows_core::IUnknown {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyCredentialAttestationResult> for ::windows_core::IInspectable {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for ::windows_core::IInspectable {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialAttestationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialAttestationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyCredentialAttestationStatus(pub i32);
impl KeyCredentialAttestationStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const TemporaryFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyCredentialAttestationStatus {}
impl ::core::clone::Clone for KeyCredentialAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialAttestationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyCredentialAttestationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialAttestationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialAttestationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: Self = Self(0i32);
    pub const FailIfExists: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyCredentialCreationOption {}
impl ::core::clone::Clone for KeyCredentialCreationOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialCreationOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyCredentialCreationOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialCreationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialCreationOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialCreationOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialCreationOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct KeyCredentialManager;
impl KeyCredentialManager {
    pub fn IsSupportedAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn RenewAttestationAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RenewAttestationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn RequestCreateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0, option: KeyCredentialCreationOption) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), option, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    pub fn OpenAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    pub fn DeleteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn IKeyCredentialManagerStatics<R, F: FnOnce(&IKeyCredentialManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyCredentialManager, IKeyCredentialManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KeyCredentialManager {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialManager";
}
#[repr(transparent)]
pub struct KeyCredentialOperationResult(::windows_core::IUnknown);
impl KeyCredentialOperationResult {
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyCredentialStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialOperationResult {}
impl ::core::fmt::Debug for KeyCredentialOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialOperationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialOperationResult;{f53786c1-5261-4cdd-976d-cc909ac71620})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
    const IID: ::windows_core::GUID = <IKeyCredentialOperationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialOperationResult";
}
impl ::core::convert::From<KeyCredentialOperationResult> for ::windows_core::IUnknown {
    fn from(value: KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for ::windows_core::IUnknown {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyCredentialOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyCredentialOperationResult> for ::windows_core::IInspectable {
    fn from(value: KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for ::windows_core::IInspectable {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyCredentialOperationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialOperationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialOperationResult {}
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(::windows_core::IUnknown);
impl KeyCredentialRetrievalResult {
    pub fn Credential(&self) -> ::windows_core::Result<KeyCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Credential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredential>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyCredentialStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialRetrievalResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialRetrievalResult {}
impl ::core::fmt::Debug for KeyCredentialRetrievalResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialRetrievalResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialRetrievalResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialRetrievalResult;{58cd7703-8d87-4249-9b58-f6598cc9644e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
    const IID: ::windows_core::GUID = <IKeyCredentialRetrievalResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialRetrievalResult";
}
impl ::core::convert::From<KeyCredentialRetrievalResult> for ::windows_core::IUnknown {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for ::windows_core::IUnknown {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyCredentialRetrievalResult> for ::windows_core::IInspectable {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for ::windows_core::IInspectable {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialRetrievalResult {}
unsafe impl ::core::marker::Sync for KeyCredentialRetrievalResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
    pub const UserPrefersPassword: Self = Self(4i32);
    pub const CredentialAlreadyExists: Self = Self(5i32);
    pub const SecurityDeviceLocked: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyCredentialStatus {}
impl ::core::clone::Clone for KeyCredentialStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyCredentialStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyCredentialStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PasswordCredential(::windows_core::IUnknown);
impl PasswordCredential {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PasswordCredential, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CreatePasswordCredential<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(resource: Param0, username: Param1, password: Param2) -> ::windows_core::Result<PasswordCredential> {
        Self::ICredentialFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreatePasswordCredential)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), username.into_param().abi(), password.into_param().abi(), result__.as_mut_ptr()).from_abi::<PasswordCredential>(result__)
        })
    }
    pub fn Resource(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Resource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resource: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResource)(::windows_core::Interface::as_raw(this), resource.into_param().abi()).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, username: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), username.into_param().abi()).ok() }
    }
    pub fn Password(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Password)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPassword<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, password: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassword)(::windows_core::Interface::as_raw(this), password.into_param().abi()).ok() }
    }
    pub fn RetrievePassword(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RetrievePassword)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn ICredentialFactory<R, F: FnOnce(&ICredentialFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PasswordCredential, ICredentialFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PasswordCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PasswordCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PasswordCredential {}
impl ::core::fmt::Debug for PasswordCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordCredential").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PasswordCredential {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredential;{6ab18989-c720-41a7-a6c1-feadb36329a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
    const IID: ::windows_core::GUID = <IPasswordCredential as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredential";
}
impl ::core::convert::From<PasswordCredential> for ::windows_core::IUnknown {
    fn from(value: PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordCredential> for ::windows_core::IUnknown {
    fn from(value: &PasswordCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PasswordCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PasswordCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PasswordCredential> for ::windows_core::IInspectable {
    fn from(value: PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordCredential> for ::windows_core::IInspectable {
    fn from(value: &PasswordCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PasswordCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PasswordCredential {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PasswordCredential {}
unsafe impl ::core::marker::Sync for PasswordCredential {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PasswordCredentialPropertyStore {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PasswordCredentialPropertyStore, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::IInspectable>>>(&self, vhnd: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PasswordCredentialPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PasswordCredentialPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PasswordCredentialPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordCredentialPropertyStore").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::RuntimeType for PasswordCredentialPropertyStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredentialPropertyStore;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PasswordCredentialPropertyStore {
    type Vtable = ::winrt_foundation::Collections::IPropertySet_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IPropertySet as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PasswordCredentialPropertyStore {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredentialPropertyStore";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PasswordCredentialPropertyStore {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PasswordCredentialPropertyStore {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PasswordCredentialPropertyStore> for ::windows_core::IUnknown {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for ::windows_core::IUnknown {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PasswordCredentialPropertyStore> for ::windows_core::IInspectable {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for ::windows_core::IInspectable {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IPropertySet {
    type Error = ::windows_core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::winrt_foundation::Collections::IPropertySet {
    type Error = ::windows_core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IPropertySet> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IPropertySet> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IPropertySet>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PasswordCredentialPropertyStore {}
#[repr(transparent)]
pub struct PasswordVault(::windows_core::IUnknown);
impl PasswordVault {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PasswordVault, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Add<'a, Param0: ::windows_core::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    pub fn Retrieve<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resource: Param0, username: Param1) -> ::windows_core::Result<PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Retrieve)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), username.into_param().abi(), result__.as_mut_ptr()).from_abi::<PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByResource<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resource: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllByResource)(::windows_core::Interface::as_raw(this), resource.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByUserName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, username: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllByUserName)(::windows_core::Interface::as_raw(this), username.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrieveAll(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveAll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
}
impl ::core::clone::Clone for PasswordVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PasswordVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PasswordVault {}
impl ::core::fmt::Debug for PasswordVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordVault").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PasswordVault {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordVault;{61fd2c0b-c8d4-48c1-a54f-bc5a64205af2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PasswordVault {
    type Vtable = IPasswordVault_Vtbl;
    const IID: ::windows_core::GUID = <IPasswordVault as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordVault";
}
impl ::core::convert::From<PasswordVault> for ::windows_core::IUnknown {
    fn from(value: PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordVault> for ::windows_core::IUnknown {
    fn from(value: &PasswordVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PasswordVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PasswordVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PasswordVault> for ::windows_core::IInspectable {
    fn from(value: PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordVault> for ::windows_core::IInspectable {
    fn from(value: &PasswordVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PasswordVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PasswordVault {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PasswordVault {}
unsafe impl ::core::marker::Sync for PasswordVault {}
#[repr(transparent)]
pub struct WebAccount(::windows_core::IUnknown);
impl WebAccount {
    pub fn WebAccountProvider(&self) -> ::windows_core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebAccountProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebAccountState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountState>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>> {
        let this = &::windows_core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPictureAsync)(::windows_core::Interface::as_raw(this), desizedsize, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    pub fn SignOutAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignOutAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn SignOutWithClientIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, clientid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignOutWithClientIdAsync)(::windows_core::Interface::as_raw(this), clientid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateWebAccount<'a, Param0: ::windows_core::IntoParam<'a, WebAccountProvider>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(webaccountprovider: Param0, username: Param1, state: WebAccountState) -> ::windows_core::Result<WebAccount> {
        Self::IWebAccountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebAccount)(::windows_core::Interface::as_raw(this), webaccountprovider.into_param().abi(), username.into_param().abi(), state, result__.as_mut_ptr()).from_abi::<WebAccount>(result__)
        })
    }
    pub fn IWebAccountFactory<R, F: FnOnce(&IWebAccountFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebAccount, IWebAccountFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccount {}
impl ::core::fmt::Debug for WebAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAccount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccount;{69473eb2-8031-49be-80bb-96cb46d99aba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebAccount {
    type Vtable = IWebAccount_Vtbl;
    const IID: ::windows_core::GUID = <IWebAccount as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccount";
}
impl ::core::convert::From<WebAccount> for ::windows_core::IUnknown {
    fn from(value: WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccount> for ::windows_core::IUnknown {
    fn from(value: &WebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccount> for ::windows_core::IInspectable {
    fn from(value: WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccount> for ::windows_core::IInspectable {
    fn from(value: &WebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccount> for IWebAccount {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAccount) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccount> for IWebAccount {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAccount) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAccount> for WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAccount> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAccount> for &WebAccount {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAccount> {
        ::core::convert::TryInto::<IWebAccount>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccount {}
unsafe impl ::core::marker::Sync for WebAccount {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebAccountPictureSize(pub i32);
impl WebAccountPictureSize {
    pub const Size64x64: Self = Self(64i32);
    pub const Size208x208: Self = Self(208i32);
    pub const Size424x424: Self = Self(424i32);
    pub const Size1080x1080: Self = Self(1080i32);
}
impl ::core::marker::Copy for WebAccountPictureSize {}
impl ::core::clone::Clone for WebAccountPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WebAccountPictureSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAccountPictureSize {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountPictureSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WebAccountProvider(::windows_core::IUnknown);
impl WebAccountProvider {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IconUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn DisplayPurpose(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayPurpose)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Authority(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Authority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<IWebAccountProvider3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn IsSystemProvider(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IWebAccountProvider4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSystemProvider)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateWebAccountProvider<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(id: Param0, displayname: Param1, iconuri: Param2) -> ::windows_core::Result<WebAccountProvider> {
        Self::IWebAccountProviderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebAccountProvider)(::windows_core::Interface::as_raw(this), id.into_param().abi(), displayname.into_param().abi(), iconuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        })
    }
    pub fn IWebAccountProviderFactory<R, F: FnOnce(&IWebAccountProviderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebAccountProvider, IWebAccountProviderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebAccountProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProvider {}
impl ::core::fmt::Debug for WebAccountProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAccountProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccountProvider;{29dcc8c3-7ab9-4a7c-a336-b942f9dbf7c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
    const IID: ::windows_core::GUID = <IWebAccountProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccountProvider";
}
impl ::core::convert::From<WebAccountProvider> for ::windows_core::IUnknown {
    fn from(value: WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProvider> for ::windows_core::IUnknown {
    fn from(value: &WebAccountProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebAccountProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebAccountProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProvider> for ::windows_core::IInspectable {
    fn from(value: WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProvider> for ::windows_core::IInspectable {
    fn from(value: &WebAccountProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebAccountProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebAccountProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebAccountProvider {}
unsafe impl ::core::marker::Sync for WebAccountProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAccountState {}
impl ::core::clone::Clone for WebAccountState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WebAccountState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAccountState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
