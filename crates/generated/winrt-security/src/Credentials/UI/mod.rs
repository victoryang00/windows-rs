#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AuthenticationProtocol(pub i32);
impl AuthenticationProtocol {
    pub const Basic: Self = Self(0i32);
    pub const Digest: Self = Self(1i32);
    pub const Ntlm: Self = Self(2i32);
    pub const Kerberos: Self = Self(3i32);
    pub const Negotiate: Self = Self(4i32);
    pub const CredSsp: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for AuthenticationProtocol {}
impl ::core::clone::Clone for AuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AuthenticationProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AuthenticationProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for AuthenticationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AuthenticationProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AuthenticationProtocol {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.AuthenticationProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CredentialPicker;
impl CredentialPicker {
    pub fn PickWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, CredentialPickerOptions>>(options: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickWithOptionsAsync)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    pub fn PickWithMessageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(targetname: Param0, message: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickWithMessageAsync)(::windows_core::Interface::as_raw(this), targetname.into_param().abi(), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    pub fn PickWithCaptionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(targetname: Param0, message: Param1, caption: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickWithCaptionAsync)(::windows_core::Interface::as_raw(this), targetname.into_param().abi(), message.into_param().abi(), caption.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    pub fn ICredentialPickerStatics<R, F: FnOnce(&ICredentialPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CredentialPicker, ICredentialPickerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CredentialPicker {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPicker";
}
#[repr(transparent)]
pub struct CredentialPickerOptions(::windows_core::IUnknown);
impl CredentialPickerOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CredentialPickerOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetCaption<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCaption)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetMessage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetErrorCode(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorCode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAuthenticationProtocol(&self, value: AuthenticationProtocol) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthenticationProtocol)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationProtocol(&self) -> ::windows_core::Result<AuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AuthenticationProtocol>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationProtocol)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AuthenticationProtocol>(result__)
        }
    }
    pub fn SetCustomAuthenticationProtocol<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomAuthenticationProtocol)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CustomAuthenticationProtocol(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CustomAuthenticationProtocol)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPreviousCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviousCredential)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PreviousCredential(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn SetAlwaysDisplayDialog(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysDisplayDialog)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysDisplayDialog(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysDisplayDialog)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCallerSavesCredential(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCallerSavesCredential)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CallerSavesCredential(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CallerSavesCredential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCredentialSaveOption(&self, value: CredentialSaveOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCredentialSaveOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CredentialSaveOption(&self) -> ::windows_core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CredentialSaveOption>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialSaveOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CredentialSaveOption>(result__)
        }
    }
}
impl ::core::clone::Clone for CredentialPickerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialPickerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialPickerOptions {}
impl ::core::fmt::Debug for CredentialPickerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPickerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CredentialPickerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerOptions;{965a0b4c-95fa-467f-992b-0b22e5859bf6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CredentialPickerOptions {
    type Vtable = ICredentialPickerOptions_Vtbl;
    const IID: ::windows_core::GUID = <ICredentialPickerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CredentialPickerOptions {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerOptions";
}
impl ::core::convert::From<CredentialPickerOptions> for ::windows_core::IUnknown {
    fn from(value: CredentialPickerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerOptions> for ::windows_core::IUnknown {
    fn from(value: &CredentialPickerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CredentialPickerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CredentialPickerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CredentialPickerOptions> for ::windows_core::IInspectable {
    fn from(value: CredentialPickerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerOptions> for ::windows_core::IInspectable {
    fn from(value: &CredentialPickerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CredentialPickerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CredentialPickerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct CredentialPickerResults(::windows_core::IUnknown);
impl CredentialPickerResults {
    pub fn ErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CredentialSaveOption(&self) -> ::windows_core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<CredentialSaveOption>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialSaveOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CredentialSaveOption>(result__)
        }
    }
    pub fn CredentialSaved(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialSaved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Credential(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Credential)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    pub fn CredentialDomainName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialDomainName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CredentialUserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialUserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CredentialPassword(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CredentialPassword)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CredentialPickerResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialPickerResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialPickerResults {}
impl ::core::fmt::Debug for CredentialPickerResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPickerResults").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CredentialPickerResults {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerResults;{1948f99a-cc30-410c-9c38-cc0884c5b3d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CredentialPickerResults {
    type Vtable = ICredentialPickerResults_Vtbl;
    const IID: ::windows_core::GUID = <ICredentialPickerResults as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CredentialPickerResults {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerResults";
}
impl ::core::convert::From<CredentialPickerResults> for ::windows_core::IUnknown {
    fn from(value: CredentialPickerResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerResults> for ::windows_core::IUnknown {
    fn from(value: &CredentialPickerResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CredentialPickerResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CredentialPickerResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CredentialPickerResults> for ::windows_core::IInspectable {
    fn from(value: CredentialPickerResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerResults> for ::windows_core::IInspectable {
    fn from(value: &CredentialPickerResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CredentialPickerResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CredentialPickerResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CredentialSaveOption(pub i32);
impl CredentialSaveOption {
    pub const Unselected: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialSaveOption {}
impl ::core::clone::Clone for CredentialSaveOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CredentialSaveOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CredentialSaveOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for CredentialSaveOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialSaveOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CredentialSaveOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.CredentialSaveOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICredentialPickerOptions {
    type Vtable = ICredentialPickerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x965a0b4c_95fa_467f_992b_0b22e5859bf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AuthenticationProtocol) -> ::windows_core::HRESULT,
    pub AuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AuthenticationProtocol) -> ::windows_core::HRESULT,
    pub SetCustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetPreviousCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPreviousCredential: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PreviousCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PreviousCredential: usize,
    pub SetAlwaysDisplayDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AlwaysDisplayDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCallerSavesCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CallerSavesCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CredentialSaveOption) -> ::windows_core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICredentialPickerResults {
    type Vtable = ICredentialPickerResults_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1948f99a_cc30_410c_9c38_cc0884c5b3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerResults_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows_core::HRESULT,
    pub CredentialSaved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Credential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Credential: usize,
    pub CredentialDomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CredentialUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CredentialPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICredentialPickerStatics {
    type Vtable = ICredentialPickerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa3a5c73_c9ea_4782_99fb_e6d7e938e12d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PickWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PickWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PickWithCaptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, caption: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserConsentVerifierStatics {
    type Vtable = IUserConsentVerifierStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf4f3f91_564c_4ddc_b8b5_973447627c65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CheckAvailabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestVerificationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserConsentVerificationResult(pub i32);
impl UserConsentVerificationResult {
    pub const Verified: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
    pub const RetriesExhausted: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl ::core::marker::Copy for UserConsentVerificationResult {}
impl ::core::clone::Clone for UserConsentVerificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserConsentVerificationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserConsentVerificationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserConsentVerificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserConsentVerificationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserConsentVerificationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerificationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct UserConsentVerifier;
impl UserConsentVerifier {
    pub fn CheckAvailabilityAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserConsentVerifierAvailability>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAvailabilityAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserConsentVerifierAvailability>>(result__)
        })
    }
    pub fn RequestVerificationAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(message: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<UserConsentVerificationResult>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestVerificationAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<UserConsentVerificationResult>>(result__)
        })
    }
    pub fn IUserConsentVerifierStatics<R, F: FnOnce(&IUserConsentVerifierStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UserConsentVerifier, IUserConsentVerifierStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for UserConsentVerifier {
    const NAME: &'static str = "Windows.Security.Credentials.UI.UserConsentVerifier";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserConsentVerifierAvailability(pub i32);
impl UserConsentVerifierAvailability {
    pub const Available: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
}
impl ::core::marker::Copy for UserConsentVerifierAvailability {}
impl ::core::clone::Clone for UserConsentVerifierAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserConsentVerifierAvailability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserConsentVerifierAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserConsentVerifierAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserConsentVerifierAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserConsentVerifierAvailability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerifierAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
