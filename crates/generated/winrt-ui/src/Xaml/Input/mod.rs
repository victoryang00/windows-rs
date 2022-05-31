#[repr(transparent)]
pub struct AccessKeyDisplayDismissedEventArgs(::windows_core::IUnknown);
impl AccessKeyDisplayDismissedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessKeyDisplayDismissedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AccessKeyDisplayDismissedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyDisplayDismissedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyDisplayDismissedEventArgs {}
impl ::core::fmt::Debug for AccessKeyDisplayDismissedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyDisplayDismissedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs;{8a610dc6-d72d-4ca8-9f66-556f35b513da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAccessKeyDisplayDismissedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[repr(transparent)]
pub struct AccessKeyDisplayRequestedEventArgs(::windows_core::IUnknown);
impl AccessKeyDisplayRequestedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessKeyDisplayRequestedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PressedKeys(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PressedKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AccessKeyDisplayRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyDisplayRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyDisplayRequestedEventArgs {}
impl ::core::fmt::Debug for AccessKeyDisplayRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyDisplayRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs;{0c079e55-13fe-4d03-a61d-e12f06567286})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAccessKeyDisplayRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[repr(transparent)]
pub struct AccessKeyInvokedEventArgs(::windows_core::IUnknown);
impl AccessKeyInvokedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessKeyInvokedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AccessKeyInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyInvokedEventArgs {}
impl ::core::fmt::Debug for AccessKeyInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs;{cfe9cd97-c718-4091-b7dd-adf1c072b1e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAccessKeyInvokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[repr(transparent)]
pub struct AccessKeyManager(::windows_core::IUnknown);
impl AccessKeyManager {
    pub fn IsDisplayModeEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisplayModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IsDisplayModeEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::windows_core::IInspectable, ::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisplayModeEnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveIsDisplayModeEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveIsDisplayModeEnabledChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn ExitDisplayMode() -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ExitDisplayMode)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn AreKeyTipsEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreKeyTipsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetAreKeyTipsEnabled)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IAccessKeyManagerStatics<R, F: FnOnce(&IAccessKeyManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessKeyManagerStatics2<R, F: FnOnce(&IAccessKeyManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AccessKeyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyManager {}
impl ::core::fmt::Debug for AccessKeyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyManager;{ecc973b0-2ee9-4b1c-98d7-6e0e816d334b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
    const IID: ::windows_core::GUID = <IAccessKeyManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyManager";
}
impl ::core::convert::From<AccessKeyManager> for ::windows_core::IUnknown {
    fn from(value: AccessKeyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows_core::IUnknown {
    fn from(value: &AccessKeyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AccessKeyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyManager> for ::windows_core::IInspectable {
    fn from(value: AccessKeyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows_core::IInspectable {
    fn from(value: &AccessKeyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AccessKeyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[repr(transparent)]
pub struct CanExecuteRequestedEventArgs(::windows_core::IUnknown);
impl CanExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn CanExecute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanExecute(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanExecute)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CanExecuteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanExecuteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanExecuteRequestedEventArgs {}
impl ::core::fmt::Debug for CanExecuteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanExecuteRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs;{c8e75256-1950-505d-993b-75907ef96830})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICanExecuteRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[repr(transparent)]
pub struct CharacterReceivedRoutedEventArgs(::windows_core::IUnknown);
impl CharacterReceivedRoutedEventArgs {
    pub fn Character(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Character)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn KeyStatus(&self) -> ::windows_core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Core::CorePhysicalKeyStatus>::zeroed();
            (::windows_core::Interface::vtable(this).KeyStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CharacterReceivedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterReceivedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterReceivedRoutedEventArgs {}
impl ::core::fmt::Debug for CharacterReceivedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterReceivedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs;{7849fd82-48e4-444d-9419-93ab8892c107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICharacterReceivedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[repr(transparent)]
pub struct ContextRequestedEventArgs(::windows_core::IUnknown);
impl ContextRequestedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContextRequestedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryGetPosition<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0, point: &mut ::winrt_foundation::Point) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPosition)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), point, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ContextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContextRequestedEventArgs {}
impl ::core::fmt::Debug for ContextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ContextRequestedEventArgs;{42618e0a-1cb6-46fb-8374-0aec68aa5e51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContextRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ContextRequestedEventArgs";
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ContextRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[repr(transparent)]
pub struct DoubleTappedEventHandler(pub ::windows_core::IUnknown);
impl DoubleTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DoubleTappedEventHandlerBox::<F> { vtable: &DoubleTappedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, DoubleTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DoubleTappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DoubleTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DoubleTappedEventHandlerBox<F> {
    const VTABLE: DoubleTappedEventHandler_Vtbl = DoubleTappedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DoubleTappedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DoubleTappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleTappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleTappedEventHandler {}
impl ::core::fmt::Debug for DoubleTappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleTappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3124d025_04a7_4d45_825e_8204a624dbf4);
}
unsafe impl ::windows_core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3124d025-04a7-4d45-825e-8204a624dbf4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct DoubleTappedRoutedEventArgs(::windows_core::IUnknown);
impl DoubleTappedRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleTappedRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPosition<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for DoubleTappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleTappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleTappedRoutedEventArgs {}
impl ::core::fmt::Debug for DoubleTappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleTappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs;{af404424-26df-44f4-8714-9359249b62d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDoubleTappedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[repr(transparent)]
pub struct ExecuteRequestedEventArgs(::windows_core::IUnknown);
impl ExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for ExecuteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExecuteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExecuteRequestedEventArgs {}
impl ::core::fmt::Debug for ExecuteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExecuteRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ExecuteRequestedEventArgs;{e07fa734-a0b6-5755-9e87-24f54cca9372})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IExecuteRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[repr(transparent)]
pub struct FindNextElementOptions(::windows_core::IUnknown);
impl FindNextElementOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FindNextElementOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SearchRoot(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SearchRoot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetSearchRoot<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSearchRoot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExclusionRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ExclusionRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetExclusionRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExclusionRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn HintRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).HintRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn SetHintRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHintRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn XYFocusNavigationStrategyOverride(&self) -> ::windows_core::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<XYFocusNavigationStrategyOverride>::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusNavigationStrategyOverride)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XYFocusNavigationStrategyOverride>(result__)
        }
    }
    pub fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetXYFocusNavigationStrategyOverride)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FindNextElementOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindNextElementOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindNextElementOptions {}
impl ::core::fmt::Debug for FindNextElementOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindNextElementOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FindNextElementOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FindNextElementOptions;{d88ae22b-46c2-41fc-897e-b5961977b89d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFindNextElementOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FindNextElementOptions";
}
impl ::core::convert::From<FindNextElementOptions> for ::windows_core::IUnknown {
    fn from(value: FindNextElementOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows_core::IUnknown {
    fn from(value: &FindNextElementOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FindNextElementOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindNextElementOptions> for ::windows_core::IInspectable {
    fn from(value: FindNextElementOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows_core::IInspectable {
    fn from(value: &FindNextElementOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FindNextElementOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: Self = Self(0i32);
    pub const Mouse: Self = Self(1i32);
    pub const Touch: Self = Self(2i32);
    pub const Pen: Self = Self(3i32);
    pub const Keyboard: Self = Self(4i32);
    pub const GameController: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusInputDeviceKind {}
impl ::core::clone::Clone for FocusInputDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusInputDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FocusInputDeviceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusInputDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusInputDeviceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusInputDeviceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FocusManager(::windows_core::IUnknown);
impl FocusManager {
    pub fn GetFocusedElement() -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        })
    }
    pub fn TryMoveFocus(focusnavigationdirection: FocusNavigationDirection) -> ::windows_core::Result<bool> {
        Self::IFocusManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocus)(::windows_core::Interface::as_raw(this), focusnavigationdirection, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn FindNextFocusableElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindNextFocusableElement)(::windows_core::Interface::as_raw(this), focusnavigationdirection, result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn FindNextFocusableElementWithHint<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(focusnavigationdirection: FocusNavigationDirection, hintrect: Param1) -> ::windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindNextFocusableElementWithHint)(::windows_core::Interface::as_raw(this), focusnavigationdirection, hintrect.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn TryMoveFocusWithOptions<'a, Param1: ::windows_core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows_core::Result<bool> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusWithOptions)(::windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn FindNextElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindNextElement)(::windows_core::Interface::as_raw(this), focusnavigationdirection, result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindFirstFocusableElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindFirstFocusableElement)(::windows_core::Interface::as_raw(this), searchscope.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindLastFocusableElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindLastFocusableElement)(::windows_core::Interface::as_raw(this), searchscope.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindNextElementWithOptions<'a, Param1: ::windows_core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindNextElementWithOptions)(::windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn TryFocusAsync<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FocusState) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryFocusAsync)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    pub fn TryMoveFocusAsync(focusnavigationdirection: FocusNavigationDirection) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusAsync)(::windows_core::Interface::as_raw(this), focusnavigationdirection, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    pub fn TryMoveFocusWithOptionsAsync<'a, Param1: ::windows_core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusWithOptionsAsync)(::windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    pub fn GotFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<FocusManagerGotFocusEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGotFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveGotFocus)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn LostFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<FocusManagerLostFocusEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLostFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLostFocus)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn GettingFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<GettingFocusEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GettingFocus)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveGettingFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveGettingFocus)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn LosingFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<LosingFocusEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LosingFocus)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLosingFocus<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLosingFocus)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn GetFocusedElement2<'a, Param0: ::windows_core::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IFocusManagerStatics7(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).GetFocusedElement)(::windows_core::Interface::as_raw(this), xamlroot.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        })
    }
    pub fn IFocusManagerStatics<R, F: FnOnce(&IFocusManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics2<R, F: FnOnce(&IFocusManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics3<R, F: FnOnce(&IFocusManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics4<R, F: FnOnce(&IFocusManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics5<R, F: FnOnce(&IFocusManagerStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics6<R, F: FnOnce(&IFocusManagerStatics6) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics6> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics7<R, F: FnOnce(&IFocusManagerStatics7) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FocusManager, IFocusManagerStatics7> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FocusManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManager {}
impl ::core::fmt::Debug for FocusManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManager;{c843f50b-3b83-4da1-9d6f-557c1169f341})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FocusManager {
    type Vtable = IFocusManager_Vtbl;
    const IID: ::windows_core::GUID = <IFocusManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManager";
}
impl ::core::convert::From<FocusManager> for ::windows_core::IUnknown {
    fn from(value: FocusManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManager> for ::windows_core::IUnknown {
    fn from(value: &FocusManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FocusManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FocusManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManager> for ::windows_core::IInspectable {
    fn from(value: FocusManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManager> for ::windows_core::IInspectable {
    fn from(value: &FocusManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FocusManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FocusManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[repr(transparent)]
pub struct FocusManagerGotFocusEventArgs(::windows_core::IUnknown);
impl FocusManagerGotFocusEventArgs {
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusManagerGotFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManagerGotFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManagerGotFocusEventArgs {}
impl ::core::fmt::Debug for FocusManagerGotFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManagerGotFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs;{97aa5d83-535b-507a-868e-62b706f06b61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFocusManagerGotFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[repr(transparent)]
pub struct FocusManagerLostFocusEventArgs(::windows_core::IUnknown);
impl FocusManagerLostFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusManagerLostFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManagerLostFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManagerLostFocusEventArgs {}
impl ::core::fmt::Debug for FocusManagerLostFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManagerLostFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs;{3e157e7a-9578-5cd3-aaa8-051b3d391978})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFocusManagerLostFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[repr(transparent)]
pub struct FocusMovementResult(::windows_core::IUnknown);
impl FocusMovementResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusMovementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusMovementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusMovementResult {}
impl ::core::fmt::Debug for FocusMovementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusMovementResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusMovementResult;{06dfead3-c2ae-44bb-bfab-9c73de8407a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
    const IID: ::windows_core::GUID = <IFocusMovementResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusMovementResult";
}
impl ::core::convert::From<FocusMovementResult> for ::windows_core::IUnknown {
    fn from(value: FocusMovementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows_core::IUnknown {
    fn from(value: &FocusMovementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FocusMovementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusMovementResult> for ::windows_core::IInspectable {
    fn from(value: FocusMovementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows_core::IInspectable {
    fn from(value: &FocusMovementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FocusMovementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: Self = Self(0i32);
    pub const Previous: Self = Self(1i32);
    pub const Up: Self = Self(2i32);
    pub const Down: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Right: Self = Self(5i32);
    pub const None: Self = Self(6i32);
}
impl ::core::marker::Copy for FocusNavigationDirection {}
impl ::core::clone::Clone for FocusNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusNavigationDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FocusNavigationDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusNavigationDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusNavigationDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusNavigationDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GettingFocusEventArgs(::windows_core::IUnknown);
impl GettingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetNewFocusedElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNewFocusedElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FocusState(&self) -> ::windows_core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::FocusState>::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusNavigationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusInputDeviceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetNewFocusedElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<IGettingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for GettingFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GettingFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GettingFocusEventArgs {}
impl ::core::fmt::Debug for GettingFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GettingFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.GettingFocusEventArgs;{fa05b9ce-c67c-4be8-8fd4-c44d67877e0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGettingFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.GettingFocusEventArgs";
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: GettingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: GettingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &GettingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[repr(transparent)]
pub struct HoldingEventHandler(pub ::windows_core::IUnknown);
impl HoldingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = HoldingEventHandlerBox::<F> { vtable: &HoldingEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, HoldingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct HoldingEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const HoldingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> HoldingEventHandlerBox<F> {
    const VTABLE: HoldingEventHandler_Vtbl = HoldingEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<HoldingEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for HoldingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingEventHandler {}
impl ::core::fmt::Debug for HoldingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecae8ccd_8e5e_4fbe_9846_30a6370afcdf);
}
unsafe impl ::windows_core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ecae8ccd-8e5e-4fbe-9846-30a6370afcdf}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct HoldingRoutedEventArgs(::windows_core::IUnknown);
impl HoldingRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HoldingRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn HoldingState(&self) -> ::windows_core::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::HoldingState>::zeroed();
            (::windows_core::Interface::vtable(this).HoldingState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::HoldingState>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPosition<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for HoldingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingRoutedEventArgs {}
impl ::core::fmt::Debug for HoldingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.HoldingRoutedEventArgs;{c246ff23-d80d-44de-8db9-0d815e269ac0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHoldingRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.HoldingRoutedEventArgs";
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyDisplayDismissedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyDisplayRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PressedKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyInvokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ca0efe6_d9c8_4ebc_b4c7_30d1838a81f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDisplayModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ExitDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManagerStatics2 {
    type Vtable = IAccessKeyManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x962bb594_2ab3_47c5_954b_7092f355f797);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AreKeyTipsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAreKeyTipsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanExecuteRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterReceivedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Character: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICommand(::windows_core::IUnknown);
impl ICommand {
    pub fn CanExecuteChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanExecuteChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CanExecute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, parameter: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(::windows_core::Interface::as_raw(this), parameter.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, parameter: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Execute)(::windows_core::Interface::as_raw(this), parameter.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ICommand> for ::windows_core::IUnknown {
    fn from(value: ICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommand> for ::windows_core::IUnknown {
    fn from(value: &ICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommand> for ::windows_core::IInspectable {
    fn from(value: ICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommand> for ::windows_core::IInspectable {
    fn from(value: &ICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommand {}
impl ::core::fmt::Debug for ICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e5af3542-ca67-4081-995b-709dd13792df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICommand {
    type Vtable = ICommand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContextRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub TryGetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, point: *mut ::winrt_foundation::Point, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleTappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExecuteRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindNextElementOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SearchRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSearchRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub SetHintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub XYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows_core::HRESULT,
    pub SetXYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManager {
    type Vtable = IFocusManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerGotFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerLostFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1eccd326_8182_4482_826a_0918e9ed9af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics2 {
    type Vtable = IFocusManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa920d761_dd87_4f31_beda_ef417fe7c04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryMoveFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics3 {
    type Vtable = IFocusManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60805ebf_b149_417d_83f1_baeb560e2a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FindNextFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindNextFocusableElementWithHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, hintrect: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics4 {
    type Vtable = IFocusManagerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29276e9c_1c6c_414a_ba1c_96efd5962bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryMoveFocusWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FindNextElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindFirstFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchscope: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindLastFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchscope: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindNextElementWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics5 {
    type Vtable = IFocusManagerStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x280edc61_207a_4d7b_b98f_ce165e1b2015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryFocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: super::FocusState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryMoveFocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryMoveFocusWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics6 {
    type Vtable = IFocusManagerStatics6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3546a1b6_20bf_5007_929d_e6d32e16afe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GettingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGettingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub LosingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLosingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics7 {
    type Vtable = IFocusManagerStatics7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95d6fa97_f0fc_5c32_b29d_07c04ec966b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamlroot: ::windows_core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusMovementResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGettingFocusEventArgs2 {
    type Vtable = IGettingFocusEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88754d7b_b4b9_4959_8bce_89bf212ed4eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGettingFocusEventArgs3 {
    type Vtable = IGettingFocusEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e024891_db3f_5e78_b75a_62bfc3510735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    #[cfg(feature = "winrt-ui")]
    pub HoldingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    HoldingState: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaExpansionBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaRotationBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaTranslationBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScope(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScope {
    type Vtable = IInputScope_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Names: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Names: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScopeName(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScopeName {
    type Vtable = IInputScopeName_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NameValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputScopeNameValue) -> ::windows_core::HRESULT,
    pub SetNameValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InputScopeNameValue) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScopeNameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a40bb52_4bd7_4e54_8617_1cda8a1eda7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    Key: usize,
    #[cfg(feature = "winrt-ui")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyRoutedEventArgs2 {
    type Vtable = IKeyRoutedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b02d57a_9634_4f14_91b2_133e42fdb3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub OriginalKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    OriginalKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyRoutedEventArgs3 {
    type Vtable = IKeyRoutedEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2779f5b4_ca41_411b_a8ef_f4fc78e78057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAccelerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    Key: usize,
    #[cfg(feature = "winrt-system")]
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    SetKey: usize,
    #[cfg(feature = "winrt-system")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    Modifiers: usize,
    #[cfg(feature = "winrt-system")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    SetModifiers: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44d88a99_4bfd_4a47_a893_515f388623f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Element: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorInvokedEventArgs2 {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbefca4b8_5907_48ee_8e21_9c969078fa11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyboardAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bd43d51_9bb3_456d_bf15_804adfb86261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ModifiersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ScopeOwnerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILosingFocusEventArgs2 {
    type Vtable = ILosingFocusEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0493fad9_c27f_469f_8e62_52b3a4f7cd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILosingFocusEventArgs3 {
    type Vtable = ILosingFocusEventArgs3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc98900bd_0b79_566e_ad1f_436fa513ae22);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Cumulative: usize,
    #[cfg(feature = "winrt-ui")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationDeltaRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Delta: usize,
    #[cfg(feature = "winrt-ui")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Cumulative: usize,
    #[cfg(feature = "winrt-ui")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpansionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExpansionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RotationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRotationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TranslationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTranslationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    #[cfg(feature = "winrt-ui")]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Delta: usize,
    #[cfg(feature = "winrt-ui")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Cumulative: usize,
    #[cfg(feature = "winrt-ui")]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationPivot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationPivotFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d05b039_3702_4396_ad9b_a825efa63a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithCenterAndRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: ::winrt_foundation::Point, radius: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    #[cfg(feature = "winrt-ui")]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Cumulative: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84c1daa7_7272_4463_b6c3_a40b9ba151fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationModes) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ManipulationModes) -> ::windows_core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Pivot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPivot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INoFocusCandidateFoundEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointer {
    type Vtable = IPointer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub IsInContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Pointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    KeyModifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetCurrentPoint: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-ui")))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerRoutedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerRoutedEventArgs2 {
    type Vtable = IPointerRoutedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0821f294_1de6_4711_ba7c_8d4b8b0911d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessKeyboardAcceleratorEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    Key: usize,
    #[cfg(feature = "winrt-system")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::VirtualKeyModifiers) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    Modifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StandardUICommandKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommand2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommand2 {
    type Vtable = IStandardUICommand2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3666069_f9e4_51eb_885b_7a620a0782ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StandardUICommandKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f1a7590_dce1_56e4_ab63_f5ce3ce4ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateInstanceWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommandStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ea87ed9_2978_5533_9b2e_6759ce88569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KindProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-devices")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_devices::Input::PointerDeviceType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub IconSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    IconSource: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetIconSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetIconSource: usize,
    #[cfg(feature = "winrt-foundation")]
    pub KeyboardAccelerators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    KeyboardAccelerators: usize,
    pub AccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CanExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NotifyCanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1eec08c3_e061_5e10_9f2a_2baa840885c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommandStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66bc457c_1a0c_58ed_876e_71533f966db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LabelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct InertiaExpansionBehavior(::windows_core::IUnknown);
impl InertiaExpansionBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredExpansion(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredExpansion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredExpansion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaExpansionBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaExpansionBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaExpansionBehavior {}
impl ::core::fmt::Debug for InertiaExpansionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaExpansionBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaExpansionBehavior;{751d87e5-8d42-44c5-965e-3cd30cc9d6f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
    const IID: ::windows_core::GUID = <IInertiaExpansionBehavior as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaExpansionBehavior";
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows_core::IUnknown {
    fn from(value: InertiaExpansionBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows_core::IUnknown {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows_core::IInspectable {
    fn from(value: InertiaExpansionBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows_core::IInspectable {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[repr(transparent)]
pub struct InertiaRotationBehavior(::windows_core::IUnknown);
impl InertiaRotationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredRotation(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRotation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaRotationBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaRotationBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaRotationBehavior {}
impl ::core::fmt::Debug for InertiaRotationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaRotationBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaRotationBehavior;{424cfb2e-bbfd-4625-ae78-20c65bf1efaf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
    const IID: ::windows_core::GUID = <IInertiaRotationBehavior as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaRotationBehavior";
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows_core::IUnknown {
    fn from(value: InertiaRotationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows_core::IUnknown {
    fn from(value: &InertiaRotationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows_core::IInspectable {
    fn from(value: InertiaRotationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows_core::IInspectable {
    fn from(value: &InertiaRotationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[repr(transparent)]
pub struct InertiaTranslationBehavior(::windows_core::IUnknown);
impl InertiaTranslationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredDisplacement(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDisplacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDisplacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaTranslationBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaTranslationBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaTranslationBehavior {}
impl ::core::fmt::Debug for InertiaTranslationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaTranslationBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaTranslationBehavior;{45d3a512-3b32-4882-a4c2-ecfa2d4b6df0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
    const IID: ::windows_core::GUID = <IInertiaTranslationBehavior as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaTranslationBehavior";
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows_core::IUnknown {
    fn from(value: InertiaTranslationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows_core::IUnknown {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows_core::IInspectable {
    fn from(value: InertiaTranslationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows_core::IInspectable {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[repr(transparent)]
pub struct InputScope(::windows_core::IUnknown);
impl InputScope {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputScope, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Names(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Names)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<InputScopeName>>(result__)
        }
    }
}
impl ::core::clone::Clone for InputScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputScope {}
impl ::core::fmt::Debug for InputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScope").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputScope {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScope;{5c0f85f3-f9d8-4220-b666-045d074d9bfa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InputScope {
    type Vtable = IInputScope_Vtbl;
    const IID: ::windows_core::GUID = <IInputScope as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScope";
}
impl ::core::convert::From<InputScope> for ::windows_core::IUnknown {
    fn from(value: InputScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScope> for ::windows_core::IUnknown {
    fn from(value: &InputScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScope> for ::windows_core::IInspectable {
    fn from(value: InputScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScope> for ::windows_core::IInspectable {
    fn from(value: &InputScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScope> for super::DependencyObject {
    fn from(value: InputScope) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputScope> for super::DependencyObject {
    fn from(value: &InputScope) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &InputScope {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[repr(transparent)]
pub struct InputScopeName(::windows_core::IUnknown);
impl InputScopeName {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputScopeName, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NameValue(&self) -> ::windows_core::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InputScopeNameValue>::zeroed();
            (::windows_core::Interface::vtable(this).NameValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputScopeNameValue>(result__)
        }
    }
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNameValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(namevalue: InputScopeNameValue) -> ::windows_core::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), namevalue, result__.as_mut_ptr()).from_abi::<InputScopeName>(result__)
        })
    }
    pub fn IInputScopeNameFactory<R, F: FnOnce(&IInputScopeNameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InputScopeName, IInputScopeNameFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputScopeName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputScopeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputScopeName {}
impl ::core::fmt::Debug for InputScopeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScopeName").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScopeName;{fd3e6997-08fb-4cba-a021-792d7589fd5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InputScopeName {
    type Vtable = IInputScopeName_Vtbl;
    const IID: ::windows_core::GUID = <IInputScopeName as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScopeName";
}
impl ::core::convert::From<InputScopeName> for ::windows_core::IUnknown {
    fn from(value: InputScopeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows_core::IUnknown {
    fn from(value: &InputScopeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScopeName> for ::windows_core::IInspectable {
    fn from(value: InputScopeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows_core::IInspectable {
    fn from(value: &InputScopeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScopeName> for super::DependencyObject {
    fn from(value: InputScopeName) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputScopeName> for super::DependencyObject {
    fn from(value: &InputScopeName) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &InputScopeName {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const EmailSmtpAddress: Self = Self(5i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const DateMonthNumber: Self = Self(23i32);
    pub const DateDayNumber: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const Digits: Self = Self(28i32);
    pub const Number: Self = Self(29i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinutesOrSeconds: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const NumericPassword: Self = Self(63i32);
    pub const NumericPin: Self = Self(64i32);
    pub const AlphanumericPin: Self = Self(65i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
}
impl ::core::marker::Copy for InputScopeNameValue {}
impl ::core::clone::Clone for InputScopeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputScopeNameValue {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InputScopeNameValue {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputScopeNameValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScopeNameValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.InputScopeNameValue;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct KeyEventHandler(pub ::windows_core::IUnknown);
impl KeyEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = KeyEventHandlerBox::<F> { vtable: &KeyEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, KeyRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct KeyEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const KeyEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> KeyEventHandlerBox<F> {
    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<KeyEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for KeyEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyEventHandler {}
impl ::core::fmt::Debug for KeyEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c63d2e5_7a0e_4e12_b96a_7715aa6ff1c8);
}
unsafe impl ::windows_core::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{7c63d2e5-7a0e-4e12-b96a-7715aa6ff1c8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct KeyRoutedEventArgs(::windows_core::IUnknown);
impl KeyRoutedEventArgs {
    #[cfg(feature = "winrt-system")]
    pub fn Key(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn KeyStatus(&self) -> ::windows_core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Core::CorePhysicalKeyStatus>::zeroed();
            (::windows_core::Interface::vtable(this).KeyStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn OriginalKey(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = &::windows_core::Interface::cast::<IKeyRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).OriginalKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IKeyRoutedEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyRoutedEventArgs {}
impl ::core::fmt::Debug for KeyRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyRoutedEventArgs;{d4cd3dfe-4079-42e9-a39a-3095d3f049c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IKeyRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyRoutedEventArgs";
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: KeyRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: KeyRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &KeyRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const Center: Self = Self(5i32);
    pub const Hidden: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyTipPlacementMode {}
impl ::core::clone::Clone for KeyTipPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyTipPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyTipPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyTipPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyTipPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyTipPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct KeyboardAccelerator(::windows_core::IUnknown);
impl KeyboardAccelerator {
    #[cfg(feature = "winrt-system")]
    pub fn Key(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn SetKey(&self, value: ::winrt_system::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKey)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn Modifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn SetModifiers(&self, value: ::winrt_system::VirtualKeyModifiers) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModifiers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScopeOwner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetScopeOwner<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScopeOwner)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Invoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveInvoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInvoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    pub fn KeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ModifiersProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ModifiersProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScopeOwnerProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScopeOwnerProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IKeyboardAcceleratorFactory<R, F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyboardAcceleratorStatics<R, F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyboardAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardAccelerator {}
impl ::core::fmt::Debug for KeyboardAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAccelerator;{92e6181e-19ae-465a-9b3c-a71ee9ea7420})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
    const IID: ::windows_core::GUID = <IKeyboardAccelerator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAccelerator";
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows_core::IUnknown {
    fn from(value: KeyboardAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows_core::IUnknown {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows_core::IInspectable {
    fn from(value: KeyboardAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows_core::IInspectable {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAccelerator> for super::DependencyObject {
    fn from(value: KeyboardAccelerator) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for super::DependencyObject {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &KeyboardAccelerator {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[repr(transparent)]
pub struct KeyboardAcceleratorInvokedEventArgs(::windows_core::IUnknown);
impl KeyboardAcceleratorInvokedEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Element(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Element)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn KeyboardAccelerator(&self) -> ::windows_core::Result<KeyboardAccelerator> {
        let this = &::windows_core::Interface::cast::<IKeyboardAcceleratorInvokedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerator)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyboardAccelerator>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyboardAcceleratorInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardAcceleratorInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardAcceleratorInvokedEventArgs {}
impl ::core::fmt::Debug for KeyboardAcceleratorInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAcceleratorInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs;{c00b03f2-04e7-4415-b17d-d76b9490de2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IKeyboardAcceleratorInvokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyboardAcceleratorPlacementMode {}
impl ::core::clone::Clone for KeyboardAcceleratorPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardAcceleratorPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyboardAcceleratorPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyboardAcceleratorPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAcceleratorPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0i32);
    pub const Cycle: Self = Self(1i32);
    pub const Once: Self = Self(2i32);
}
impl ::core::marker::Copy for KeyboardNavigationMode {}
impl ::core::clone::Clone for KeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KeyboardNavigationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardNavigationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardNavigationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct LosingFocusEventArgs(::windows_core::IUnknown);
impl LosingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetNewFocusedElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNewFocusedElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FocusState(&self) -> ::windows_core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::FocusState>::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusNavigationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusInputDeviceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetNewFocusedElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<ILosingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for LosingFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LosingFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LosingFocusEventArgs {}
impl ::core::fmt::Debug for LosingFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LosingFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.LosingFocusEventArgs;{f9f683c7-d789-472b-aa93-6d4105e6dabe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILosingFocusEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.LosingFocusEventArgs";
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: LosingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: LosingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &LosingFocusEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[repr(transparent)]
pub struct ManipulationCompletedEventHandler(pub ::windows_core::IUnknown);
impl ManipulationCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationCompletedEventHandlerBox::<F> { vtable: &ManipulationCompletedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ManipulationCompletedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationCompletedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ManipulationCompletedEventHandlerBox<F> {
    const VTABLE: ManipulationCompletedEventHandler_Vtbl = ManipulationCompletedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationCompletedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedEventHandler {}
impl ::core::fmt::Debug for ManipulationCompletedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38ef4b0f_14f8_42df_9a1e_a4bcc4af77f4);
}
unsafe impl ::windows_core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{38ef4b0f-14f8-42df-9a1e-a4bcc4af77f4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ManipulationCompletedRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationCompletedRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationCompletedRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInertial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Velocities(&self) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationCompletedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationCompletedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs;{b5ad9b23-2f41-498e-8319-015ee8a75346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationCompletedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[repr(transparent)]
pub struct ManipulationDeltaEventHandler(pub ::windows_core::IUnknown);
impl ManipulationDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationDeltaEventHandlerBox::<F> { vtable: &ManipulationDeltaEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ManipulationDeltaRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationDeltaEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationDeltaEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ManipulationDeltaEventHandlerBox<F> {
    const VTABLE: ManipulationDeltaEventHandler_Vtbl = ManipulationDeltaEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationDeltaEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationDeltaEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationDeltaEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationDeltaEventHandler {}
impl ::core::fmt::Debug for ManipulationDeltaEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationDeltaEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa1160cb_dfb9_4c56_abdc_711b63c8eb94);
}
unsafe impl ::windows_core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{aa1160cb-dfb9-4c56-abdc-711b63c8eb94}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ManipulationDeltaRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationDeltaRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationDeltaRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInertial)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Delta(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Velocities(&self) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ManipulationDeltaRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationDeltaRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationDeltaRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationDeltaRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationDeltaRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs;{400d5794-4c6f-491d-82d6-3517109399c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationDeltaRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventHandler(pub ::windows_core::IUnknown);
impl ManipulationInertiaStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationInertiaStartingEventHandlerBox::<F> { vtable: &ManipulationInertiaStartingEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ManipulationInertiaStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationInertiaStartingEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationInertiaStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ManipulationInertiaStartingEventHandlerBox<F> {
    const VTABLE: ManipulationInertiaStartingEventHandler_Vtbl = ManipulationInertiaStartingEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationInertiaStartingEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingEventHandler {}
impl ::core::fmt::Debug for ManipulationInertiaStartingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd39d6322_7c9c_481b_827b_c8b2d9bb6fc7);
}
unsafe impl ::windows_core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d39d6322-7c9c-481b-827b-c8b2d9bb6fc7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ManipulationInertiaStartingRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationInertiaStartingRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationInertiaStartingRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn ExpansionBehavior(&self) -> ::windows_core::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpansionBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InertiaExpansionBehavior>(result__)
        }
    }
    pub fn SetExpansionBehavior<'a, Param0: ::windows_core::IntoParam<'a, InertiaExpansionBehavior>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpansionBehavior)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RotationBehavior(&self) -> ::windows_core::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotationBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InertiaRotationBehavior>(result__)
        }
    }
    pub fn SetRotationBehavior<'a, Param0: ::windows_core::IntoParam<'a, InertiaRotationBehavior>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationBehavior)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TranslationBehavior(&self) -> ::windows_core::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TranslationBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InertiaTranslationBehavior>(result__)
        }
    }
    pub fn SetTranslationBehavior<'a, Param0: ::windows_core::IntoParam<'a, InertiaTranslationBehavior>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslationBehavior)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Delta(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Velocities(&self) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationVelocities>::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationInertiaStartingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs;{246a91a9-ca43-4c0b-acef-81e8b8147520})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationInertiaStartingRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: Self = Self(0u32);
    pub const TranslateX: Self = Self(1u32);
    pub const TranslateY: Self = Self(2u32);
    pub const TranslateRailsX: Self = Self(4u32);
    pub const TranslateRailsY: Self = Self(8u32);
    pub const Rotate: Self = Self(16u32);
    pub const Scale: Self = Self(32u32);
    pub const TranslateInertia: Self = Self(64u32);
    pub const RotateInertia: Self = Self(128u32);
    pub const ScaleInertia: Self = Self(256u32);
    pub const All: Self = Self(65535u32);
    pub const System: Self = Self(65536u32);
}
impl ::core::marker::Copy for ManipulationModes {}
impl ::core::clone::Clone for ManipulationModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManipulationModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ManipulationModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ManipulationModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.ManipulationModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ManipulationPivot(::windows_core::IUnknown);
impl ManipulationPivot {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationPivot, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Center(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetCenter<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCenter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Radius(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Radius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRadius)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstanceWithCenterAndRadius<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(center: Param0, radius: f64) -> ::windows_core::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithCenterAndRadius)(::windows_core::Interface::as_raw(this), center.into_param().abi(), radius, result__.as_mut_ptr()).from_abi::<ManipulationPivot>(result__)
        })
    }
    pub fn IManipulationPivotFactory<R, F: FnOnce(&IManipulationPivotFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationPivot, IManipulationPivotFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ManipulationPivot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationPivot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationPivot {}
impl ::core::fmt::Debug for ManipulationPivot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationPivot").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationPivot;{2e3838a5-e6c2-4998-82ac-18748b141666})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationPivot as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationPivot";
}
impl ::core::convert::From<ManipulationPivot> for ::windows_core::IUnknown {
    fn from(value: ManipulationPivot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows_core::IUnknown {
    fn from(value: &ManipulationPivot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationPivot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationPivot> for ::windows_core::IInspectable {
    fn from(value: ManipulationPivot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows_core::IInspectable {
    fn from(value: &ManipulationPivot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationPivot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[repr(transparent)]
pub struct ManipulationStartedEventHandler(pub ::windows_core::IUnknown);
impl ManipulationStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationStartedEventHandlerBox::<F> { vtable: &ManipulationStartedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ManipulationStartedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationStartedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationStartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ManipulationStartedEventHandlerBox<F> {
    const VTABLE: ManipulationStartedEventHandler_Vtbl = ManipulationStartedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationStartedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedEventHandler {}
impl ::core::fmt::Debug for ManipulationStartedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf88345f8_e0a3_4be2_b90c_dc20e6d8beb0);
}
unsafe impl ::windows_core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f88345f8-e0a3-4be2-b90c-dc20e6d8beb0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ManipulationStartedRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationStartedRoutedEventArgs {
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Input::ManipulationDelta>::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn new() -> ::windows_core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    pub fn IManipulationStartedRoutedEventArgsFactory<R, F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ManipulationStartedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs;{5db1aa05-9f80-48b6-ae6c-4f119de8ff13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationStartedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[repr(transparent)]
pub struct ManipulationStartingEventHandler(pub ::windows_core::IUnknown);
impl ManipulationStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationStartingEventHandlerBox::<F> { vtable: &ManipulationStartingEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ManipulationStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationStartingEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ManipulationStartingEventHandlerBox<F> {
    const VTABLE: ManipulationStartingEventHandler_Vtbl = ManipulationStartingEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartingEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationStartingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartingEventHandler {}
impl ::core::fmt::Debug for ManipulationStartingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10d0b04e_bfe4_42cb_823c_3fecd8770ef8);
}
unsafe impl ::windows_core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{10d0b04e-bfe4-42cb-823c-3fecd8770ef8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ManipulationStartingRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationStartingRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ManipulationStartingRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows_core::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ManipulationModes>::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationModes>(result__)
        }
    }
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetContainer<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContainer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Pivot(&self) -> ::windows_core::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Pivot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ManipulationPivot>(result__)
        }
    }
    pub fn SetPivot<'a, Param0: ::windows_core::IntoParam<'a, ManipulationPivot>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPivot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ManipulationStartingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartingRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs;{18d636b7-53a4-4c15-a498-f3a9ca212a42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IManipulationStartingRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[repr(transparent)]
pub struct NoFocusCandidateFoundEventArgs(::windows_core::IUnknown);
impl NoFocusCandidateFoundEventArgs {
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusNavigationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FocusInputDeviceKind>::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for NoFocusCandidateFoundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NoFocusCandidateFoundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NoFocusCandidateFoundEventArgs {}
impl ::core::fmt::Debug for NoFocusCandidateFoundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NoFocusCandidateFoundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs;{ec3601a7-1007-48f9-b6b3-ed0bea53937d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <INoFocusCandidateFoundEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows_core::IUnknown {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows_core::IInspectable {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[repr(transparent)]
pub struct Pointer(::windows_core::IUnknown);
impl Pointer {
    pub fn PointerId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).PointerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn IsInContact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInContact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsInRange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for Pointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Pointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Pointer {}
impl ::core::fmt::Debug for Pointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Pointer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Pointer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.Pointer;{5ee8f39f-747d-4171-90e6-cd37a9dffb11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Pointer {
    type Vtable = IPointer_Vtbl;
    const IID: ::windows_core::GUID = <IPointer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Pointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.Pointer";
}
impl ::core::convert::From<Pointer> for ::windows_core::IUnknown {
    fn from(value: Pointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pointer> for ::windows_core::IUnknown {
    fn from(value: &Pointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Pointer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Pointer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Pointer> for ::windows_core::IInspectable {
    fn from(value: Pointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pointer> for ::windows_core::IInspectable {
    fn from(value: &Pointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Pointer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Pointer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[repr(transparent)]
pub struct PointerEventHandler(pub ::windows_core::IUnknown);
impl PointerEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PointerEventHandlerBox::<F> { vtable: &PointerEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, PointerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PointerEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PointerEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> PointerEventHandlerBox<F> {
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<PointerEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for PointerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerEventHandler {}
impl ::core::fmt::Debug for PointerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4385929_c004_4bcf_8970_359486e39f88);
}
unsafe impl ::windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e4385929-c004-4bcf-8970-359486e39f88}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PointerRoutedEventArgs(::windows_core::IUnknown);
impl PointerRoutedEventArgs {
    pub fn Pointer(&self) -> ::windows_core::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Pointer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Pointer>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn KeyModifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).KeyModifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetCurrentPoint<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<super::super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentPoint)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Input::PointerPoint>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-ui"))]
    pub fn GetIntermediatePoints<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::super::Input::PointerPoint>>(result__)
        }
    }
    pub fn IsGenerated(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPointerRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGenerated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerRoutedEventArgs {}
impl ::core::fmt::Debug for PointerRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.PointerRoutedEventArgs;{da628f0a-9752-49e2-bde2-49eccab9194d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPointerRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.PointerRoutedEventArgs";
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PointerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PointerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &PointerRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
pub struct ProcessKeyboardAcceleratorEventArgs(::windows_core::IUnknown);
impl ProcessKeyboardAcceleratorEventArgs {
    #[cfg(feature = "winrt-system")]
    pub fn Key(&self) -> ::windows_core::Result<::winrt_system::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKey>::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn Modifiers(&self) -> ::windows_core::Result<::winrt_system::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::VirtualKeyModifiers>::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ProcessKeyboardAcceleratorEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessKeyboardAcceleratorEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessKeyboardAcceleratorEventArgs {}
impl ::core::fmt::Debug for ProcessKeyboardAcceleratorEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessKeyboardAcceleratorEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs;{fb79c216-972b-440c-9b83-2b4198dcf09d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProcessKeyboardAcceleratorEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[repr(transparent)]
pub struct RightTappedEventHandler(pub ::windows_core::IUnknown);
impl RightTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RightTappedEventHandlerBox::<F> { vtable: &RightTappedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, RightTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RightTappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RightTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RightTappedEventHandlerBox<F> {
    const VTABLE: RightTappedEventHandler_Vtbl = RightTappedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<RightTappedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for RightTappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedEventHandler {}
impl ::core::fmt::Debug for RightTappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2532a062_f447_4950_9c46_f1e34a2c2238);
}
unsafe impl ::windows_core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2532a062-f447-4950-9c46-f1e34a2c2238}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct RightTappedRoutedEventArgs(::windows_core::IUnknown);
impl RightTappedRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RightTappedRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPosition<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for RightTappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedRoutedEventArgs {}
impl ::core::fmt::Debug for RightTappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.RightTappedRoutedEventArgs;{6834869d-7bd5-4033-b237-172f79abe393})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRightTappedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
pub struct StandardUICommand(::windows_core::IUnknown);
impl StandardUICommand {
    pub fn Kind(&self) -> ::windows_core::Result<StandardUICommandKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StandardUICommandKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StandardUICommandKind>(result__)
        }
    }
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IStandardUICommand2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn new() -> ::windows_core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn CreateInstanceWithKind(kind: StandardUICommandKind) -> ::windows_core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithKind)(::windows_core::Interface::as_raw(this), kind, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn CreateInstanceWithKind_compose<T: ::windows_core::Compose>(kind: StandardUICommandKind, compose: T) -> ::windows_core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithKind)(::windows_core::Interface::as_raw(this), kind, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn KindProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KindProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IStandardUICommandFactory<R, F: FnOnce(&IStandardUICommandFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StandardUICommand, IStandardUICommandFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardUICommandStatics<R, F: FnOnce(&IStandardUICommandStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StandardUICommand, IStandardUICommandStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StandardUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StandardUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StandardUICommand {}
impl ::core::fmt::Debug for StandardUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StandardUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.StandardUICommand;{d2bf7f43-0504-52d0-8aa6-0cb0f756eb27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
    const IID: ::windows_core::GUID = <IStandardUICommand as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.StandardUICommand";
}
impl ::core::convert::From<StandardUICommand> for ::windows_core::IUnknown {
    fn from(value: StandardUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows_core::IUnknown {
    fn from(value: &StandardUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StandardUICommand> for ::windows_core::IInspectable {
    fn from(value: StandardUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows_core::IInspectable {
    fn from(value: &StandardUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StandardUICommand> for ICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: StandardUICommand) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StandardUICommand> for ICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: &StandardUICommand) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommand> for StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ICommand> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<StandardUICommand> for XamlUICommand {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for XamlUICommand {
    fn from(value: &StandardUICommand) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlUICommand> for StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, XamlUICommand> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, XamlUICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, XamlUICommand> {
        ::windows_core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(self))
    }
}
impl ::core::convert::From<StandardUICommand> for super::DependencyObject {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for super::DependencyObject {
    fn from(value: &StandardUICommand) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &StandardUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: Self = Self(0i32);
    pub const Cut: Self = Self(1i32);
    pub const Copy: Self = Self(2i32);
    pub const Paste: Self = Self(3i32);
    pub const SelectAll: Self = Self(4i32);
    pub const Delete: Self = Self(5i32);
    pub const Share: Self = Self(6i32);
    pub const Save: Self = Self(7i32);
    pub const Open: Self = Self(8i32);
    pub const Close: Self = Self(9i32);
    pub const Pause: Self = Self(10i32);
    pub const Play: Self = Self(11i32);
    pub const Stop: Self = Self(12i32);
    pub const Forward: Self = Self(13i32);
    pub const Backward: Self = Self(14i32);
    pub const Undo: Self = Self(15i32);
    pub const Redo: Self = Self(16i32);
}
impl ::core::marker::Copy for StandardUICommandKind {}
impl ::core::clone::Clone for StandardUICommandKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StandardUICommandKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StandardUICommandKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for StandardUICommandKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StandardUICommandKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.StandardUICommandKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TappedEventHandler(pub ::windows_core::IUnknown);
impl TappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TappedEventHandlerBox::<F> { vtable: &TappedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, TappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct TappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const TappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> TappedEventHandlerBox<F> {
    const VTABLE: TappedEventHandler_Vtbl = TappedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<TappedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for TappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedEventHandler {}
impl ::core::fmt::Debug for TappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68d940cc_9ff0_49ce_b141_3f07ec477b97);
}
unsafe impl ::windows_core::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{68d940cc-9ff0-49ce-b141-3f07ec477b97}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct TappedRoutedEventArgs(::windows_core::IUnknown);
impl TappedRoutedEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TappedRoutedEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<::winrt_devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_devices::Input::PointerDeviceType>::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPosition<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(::windows_core::Interface::as_raw(this), relativeto.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for TappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedRoutedEventArgs {}
impl ::core::fmt::Debug for TappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.TappedRoutedEventArgs;{a099e6be-e624-459a-bb1d-e05c73e2cc66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITappedRoutedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.TappedRoutedEventArgs";
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: TappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: TappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::RoutedEventArgs> for &TappedRoutedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for XYFocusKeyboardNavigationMode {}
impl ::core::clone::Clone for XYFocusKeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusKeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XYFocusKeyboardNavigationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusKeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusKeyboardNavigationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0i32);
    pub const Projection: Self = Self(1i32);
    pub const NavigationDirectionDistance: Self = Self(2i32);
    pub const RectilinearDistance: Self = Self(3i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategy {}
impl ::core::clone::Clone for XYFocusNavigationStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XYFocusNavigationStrategy {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategy").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Projection: Self = Self(2i32);
    pub const NavigationDirectionDistance: Self = Self(3i32);
    pub const RectilinearDistance: Self = Self(4i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategyOverride {}
impl ::core::clone::Clone for XYFocusNavigationStrategyOverride {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategyOverride {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XYFocusNavigationStrategyOverride {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategyOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategyOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XamlUICommand(::windows_core::IUnknown);
impl XamlUICommand {
    pub fn CanExecuteChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanExecuteChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CanExecute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, parameter: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(::windows_core::Interface::as_raw(this), parameter.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, parameter: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Execute)(::windows_core::Interface::as_raw(this), parameter.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn IconSource(&self) -> ::windows_core::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetIconSource<'a, Param0: ::windows_core::IntoParam<'a, super::Controls::IconSource>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIconSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn KeyboardAccelerators(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<KeyboardAccelerator>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerators)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAccessKey)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Command(&self) -> ::windows_core::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Command)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ICommand>(result__)
        }
    }
    pub fn SetCommand<'a, Param0: ::windows_core::IntoParam<'a, ICommand>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommand)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExecuteRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveExecuteRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveExecuteRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CanExecuteRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanExecuteRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanExecuteRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NotifyCanExecuteChanged(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyCanExecuteChanged)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn new() -> ::windows_core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<XamlUICommand>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<XamlUICommand>(result__)
        })
    }
    pub fn LabelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LabelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IconSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyboardAcceleratorsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn DescriptionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DescriptionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CommandProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IXamlUICommandFactory<R, F: FnOnce(&IXamlUICommandFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlUICommand, IXamlUICommandFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUICommandStatics<R, F: FnOnce(&IXamlUICommandStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<XamlUICommand, IXamlUICommandStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlUICommand {}
impl ::core::fmt::Debug for XamlUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.XamlUICommand;{8494f8d4-ead1-5f01-ad2e-a8cad4f9dc0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
    const IID: ::windows_core::GUID = <IXamlUICommand as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.XamlUICommand";
}
impl ::core::convert::From<XamlUICommand> for ::windows_core::IUnknown {
    fn from(value: XamlUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows_core::IUnknown {
    fn from(value: &XamlUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlUICommand> for ::windows_core::IInspectable {
    fn from(value: XamlUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows_core::IInspectable {
    fn from(value: &XamlUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XamlUICommand> for ICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: XamlUICommand) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XamlUICommand> for ICommand {
    type Error = ::windows_core::Error;
    fn try_from(value: &XamlUICommand) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommand> for XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ICommand> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommand> for &XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<XamlUICommand> for super::DependencyObject {
    fn from(value: XamlUICommand) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&XamlUICommand> for super::DependencyObject {
    fn from(value: &XamlUICommand) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &XamlUICommand {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
