#[repr(transparent)]
pub struct AppExtension(::windows_core::IUnknown);
impl AppExtension {
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
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows_core::Result<super::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AppInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetExtensionPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetExtensionPropertiesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetPublicFolderAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPublicFolderAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppExtension2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtension {}
impl ::core::fmt::Debug for AppExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtension {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtension;{8450902c-15ed-4faf-93ea-2237bbf8cbd6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtension {
    type Vtable = IAppExtension_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtension as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtension {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtension";
}
impl ::core::convert::From<AppExtension> for ::windows_core::IUnknown {
    fn from(value: AppExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtension> for ::windows_core::IUnknown {
    fn from(value: &AppExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtension> for ::windows_core::IInspectable {
    fn from(value: AppExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtension> for ::windows_core::IInspectable {
    fn from(value: &AppExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtension {}
unsafe impl ::core::marker::Sync for AppExtension {}
#[repr(transparent)]
pub struct AppExtensionCatalog(::windows_core::IUnknown);
impl AppExtensionCatalog {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestRemovePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRemovePackageAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PackageInstalled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageInstalled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageInstalled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageInstalled)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdating<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageUpdating)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdating<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUpdating)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PackageUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUpdated<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PackageUninstalling<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageUninstalling)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageUninstalling<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUninstalling)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PackageStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePackageStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Open<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appextensionname: Param0) -> ::windows_core::Result<AppExtensionCatalog> {
        Self::IAppExtensionCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Open)(::windows_core::Interface::as_raw(this), appextensionname.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppExtensionCatalog>(result__)
        })
    }
    pub fn IAppExtensionCatalogStatics<R, F: FnOnce(&IAppExtensionCatalogStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppExtensionCatalog, IAppExtensionCatalogStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppExtensionCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionCatalog {}
impl ::core::fmt::Debug for AppExtensionCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionCatalog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionCatalog {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionCatalog;{97872032-8426-4ad1-9084-92e88c2da200})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionCatalog as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionCatalog";
}
impl ::core::convert::From<AppExtensionCatalog> for ::windows_core::IUnknown {
    fn from(value: AppExtensionCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionCatalog> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionCatalog> for ::windows_core::IInspectable {
    fn from(value: AppExtensionCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionCatalog> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct AppExtensionPackageInstalledEventArgs(::windows_core::IUnknown);
impl AppExtensionPackageInstalledEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Extensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AppExtension>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageInstalledEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageInstalledEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageInstalledEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageInstalledEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageInstalledEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionPackageInstalledEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs;{39e59234-3351-4a8d-9745-e7d3dd45bc48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageInstalledEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageInstalledEventArgs";
}
impl ::core::convert::From<AppExtensionPackageInstalledEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppExtensionPackageInstalledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageInstalledEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionPackageInstalledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionPackageInstalledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionPackageInstalledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionPackageInstalledEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppExtensionPackageInstalledEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageInstalledEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionPackageInstalledEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionPackageInstalledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionPackageInstalledEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtensionPackageInstalledEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageInstalledEventArgs {}
#[repr(transparent)]
pub struct AppExtensionPackageStatusChangedEventArgs(::windows_core::IUnknown);
impl AppExtensionPackageStatusChangedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageStatusChangedEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionPackageStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs;{1ce17433-1153-44fd-87b1-8ae1050303df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageStatusChangedEventArgs";
}
impl ::core::convert::From<AppExtensionPackageStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppExtensionPackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionPackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionPackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionPackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionPackageStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppExtensionPackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionPackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionPackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionPackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtensionPackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageStatusChangedEventArgs {}
#[repr(transparent)]
pub struct AppExtensionPackageUninstallingEventArgs(::windows_core::IUnknown);
impl AppExtensionPackageUninstallingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUninstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUninstallingEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUninstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUninstallingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionPackageUninstallingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs;{60f160c5-171e-40ff-ae98-ab2c20dd4d75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUninstallingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUninstallingEventArgs";
}
impl ::core::convert::From<AppExtensionPackageUninstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppExtensionPackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUninstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionPackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionPackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionPackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionPackageUninstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppExtensionPackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUninstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionPackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionPackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionPackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtensionPackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUninstallingEventArgs {}
#[repr(transparent)]
pub struct AppExtensionPackageUpdatedEventArgs(::windows_core::IUnknown);
impl AppExtensionPackageUpdatedEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Extensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Extensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<AppExtension>>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUpdatedEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionPackageUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs;{3a83c43f-797e-44b5-ba24-a4c8b5a543d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatedEventArgs";
}
impl ::core::convert::From<AppExtensionPackageUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppExtensionPackageUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionPackageUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionPackageUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionPackageUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionPackageUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppExtensionPackageUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionPackageUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionPackageUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionPackageUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatedEventArgs {}
#[repr(transparent)]
pub struct AppExtensionPackageUpdatingEventArgs(::windows_core::IUnknown);
impl AppExtensionPackageUpdatingEventArgs {
    pub fn AppExtensionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppExtensionName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<super::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Package>(result__)
        }
    }
}
impl ::core::clone::Clone for AppExtensionPackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppExtensionPackageUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppExtensionPackageUpdatingEventArgs {}
impl ::core::fmt::Debug for AppExtensionPackageUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExtensionPackageUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExtensionPackageUpdatingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs;{7ed59329-1a65-4800-a700-b321009e306a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppExtensionPackageUpdatingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.AppExtensionPackageUpdatingEventArgs";
}
impl ::core::convert::From<AppExtensionPackageUpdatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppExtensionPackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUpdatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppExtensionPackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppExtensionPackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppExtensionPackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppExtensionPackageUpdatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppExtensionPackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppExtensionPackageUpdatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppExtensionPackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppExtensionPackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppExtensionPackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppExtensionPackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for AppExtensionPackageUpdatingEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtension(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtension {
    type Vtable = IAppExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8450902c_15ed_4faf_93ea_2237bbf8cbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetExtensionPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetExtensionPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetPublicFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetPublicFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtension2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtension2 {
    type Vtable = IAppExtension2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab3b15f0_14f9_4b9f_9419_a349a242ef38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtension2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionCatalog(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionCatalog {
    type Vtable = IAppExtensionCatalog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97872032_8426_4ad1_9084_92e88c2da200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionCatalog_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRemovePackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PackageInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageInstalled: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdating: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageUninstalling: usize,
    #[cfg(feature = "Foundation")]
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PackageStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePackageStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionCatalogStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionCatalogStatics {
    type Vtable = IAppExtensionCatalogStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c36668a_5f18_4f0b_9ce5_cab61d196f11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionCatalogStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appextensionname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageInstalledEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionPackageInstalledEventArgs {
    type Vtable = IAppExtensionPackageInstalledEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39e59234_3351_4a8d_9745_e7d3dd45bc48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageInstalledEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionPackageStatusChangedEventArgs {
    type Vtable = IAppExtensionPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ce17433_1153_44fd_87b1_8ae1050303df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUninstallingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionPackageUninstallingEventArgs {
    type Vtable = IAppExtensionPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60f160c5_171e_40ff_ae98_ab2c20dd4d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionPackageUpdatedEventArgs {
    type Vtable = IAppExtensionPackageUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a83c43f_797e_44b5_ba24_a4c8b5a543d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Extensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppExtensionPackageUpdatingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExtensionPackageUpdatingEventArgs {
    type Vtable = IAppExtensionPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ed59329_1a65_4800_a700_b321009e306a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExtensionPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
