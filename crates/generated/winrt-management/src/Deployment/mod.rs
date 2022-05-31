#[cfg(feature = "Preview")]
pub mod Preview;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: Self = Self(0u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetAppShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const LimitToExistingPackages: Self = Self(512u32);
}
impl ::core::marker::Copy for AddPackageByAppInstallerOptions {}
impl ::core::clone::Clone for AddPackageByAppInstallerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddPackageByAppInstallerOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AddPackageByAppInstallerOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddPackageByAppInstallerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageByAppInstallerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AddPackageByAppInstallerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.AddPackageByAppInstallerOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AddPackageOptions(::windows_core::IUnknown);
impl AddPackageOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AddPackageOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DependencyPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetTargetVolume<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetVolume)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RelatedPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelatedPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn ExternalLocationUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExternalLocationUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetExternalLocationUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExternalLocationUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows_core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StubPackageOption>::zeroed();
            (::windows_core::Interface::vtable(this).StubPackageOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StubPackageOption>(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStubPackageOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeveloperMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeveloperMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceTargetAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceTargetAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InstallAllResources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallAllResources)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredContentGroupOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiredContentGroupOnly)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetainFilesOnFailure(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RetainFilesOnFailure)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRetainFilesOnFailure)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StageInPlace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStageInPlace)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowUnsigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowUnsigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AddPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPackageOptions {}
impl ::core::fmt::Debug for AddPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AddPackageOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AddPackageOptions;{05cee018-f68f-422b-95a4-66679ec77fc0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AddPackageOptions {
    type Vtable = IAddPackageOptions_Vtbl;
    const IID: ::windows_core::GUID = <IAddPackageOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AddPackageOptions";
}
impl ::core::convert::From<AddPackageOptions> for ::windows_core::IUnknown {
    fn from(value: AddPackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPackageOptions> for ::windows_core::IUnknown {
    fn from(value: &AddPackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AddPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddPackageOptions> for ::windows_core::IInspectable {
    fn from(value: AddPackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPackageOptions> for ::windows_core::IInspectable {
    fn from(value: &AddPackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AddPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AddPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AddPackageOptions {}
unsafe impl ::core::marker::Sync for AddPackageOptions {}
#[repr(transparent)]
pub struct AppInstallerManager(::windows_core::IUnknown);
impl AppInstallerManager {
    pub fn SetAutoUpdateSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, AutoUpdateSettingsOptions>>(&self, packagefamilyname: Param0, appinstallerinfo: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoUpdateSettings)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), appinstallerinfo.into_param().abi()).ok() }
    }
    pub fn ClearAutoUpdateSettings<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearAutoUpdateSettings)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi()).ok() }
    }
    pub fn PauseAutoUpdatesUntil<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, packagefamilyname: Param0, datetime: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PauseAutoUpdatesUntil)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), datetime.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallerManager>(result__)
        })
    }
    pub fn GetForSystem() -> ::windows_core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallerManager>(result__)
        })
    }
    pub fn IAppInstallerManagerStatics<R, F: FnOnce(&IAppInstallerManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppInstallerManager, IAppInstallerManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInstallerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallerManager {}
impl ::core::fmt::Debug for AppInstallerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallerManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AppInstallerManager;{e7ee21c3-2103-53ee-9b18-68afeab0033d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallerManager {
    type Vtable = IAppInstallerManager_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallerManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.AppInstallerManager";
}
impl ::core::convert::From<AppInstallerManager> for ::windows_core::IUnknown {
    fn from(value: AppInstallerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerManager> for ::windows_core::IUnknown {
    fn from(value: &AppInstallerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallerManager> for ::windows_core::IInspectable {
    fn from(value: AppInstallerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerManager> for ::windows_core::IInspectable {
    fn from(value: &AppInstallerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallerManager {}
unsafe impl ::core::marker::Sync for AppInstallerManager {}
#[repr(transparent)]
pub struct AutoUpdateSettingsOptions(::windows_core::IUnknown);
impl AutoUpdateSettingsOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutoUpdateSettingsOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Version(&self) -> ::windows_core::Result<::winrt_applicationmodel::PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::PackageVersion>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::PackageVersion>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::PackageVersion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVersion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AppInstallerUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallerUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetAppInstallerUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppInstallerUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OnLaunch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).OnLaunch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetOnLaunch(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOnLaunch)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HoursBetweenUpdateChecks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHoursBetweenUpdateChecks)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowPrompt(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowPrompt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShowPrompt(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowPrompt)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateBlocksActivation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateBlocksActivation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdateBlocksActivation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticBackgroundTask(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticBackgroundTask)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticBackgroundTask)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoRepairEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoRepairEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAutoRepairEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdateUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RepairUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RepairUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DependencyPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn CreateFromAppInstallerInfo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::AppInstallerInfo>>(appinstallerinfo: Param0) -> ::windows_core::Result<AutoUpdateSettingsOptions> {
        Self::IAutoUpdateSettingsOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAppInstallerInfo)(::windows_core::Interface::as_raw(this), appinstallerinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<AutoUpdateSettingsOptions>(result__)
        })
    }
    pub fn IAutoUpdateSettingsOptionsStatics<R, F: FnOnce(&IAutoUpdateSettingsOptionsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutoUpdateSettingsOptions, IAutoUpdateSettingsOptionsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutoUpdateSettingsOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutoUpdateSettingsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoUpdateSettingsOptions {}
impl ::core::fmt::Debug for AutoUpdateSettingsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSettingsOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutoUpdateSettingsOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.AutoUpdateSettingsOptions;{67491d87-35e1-512a-8968-1ae88d1be6d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_Vtbl;
    const IID: ::windows_core::GUID = <IAutoUpdateSettingsOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AutoUpdateSettingsOptions";
}
impl ::core::convert::From<AutoUpdateSettingsOptions> for ::windows_core::IUnknown {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoUpdateSettingsOptions> for ::windows_core::IUnknown {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutoUpdateSettingsOptions> for ::windows_core::IInspectable {
    fn from(value: AutoUpdateSettingsOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoUpdateSettingsOptions> for ::windows_core::IInspectable {
    fn from(value: &AutoUpdateSettingsOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutoUpdateSettingsOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutoUpdateSettingsOptions {}
unsafe impl ::core::marker::Sync for AutoUpdateSettingsOptions {}
#[repr(transparent)]
pub struct CreateSharedPackageContainerOptions(::windows_core::IUnknown);
impl CreateSharedPackageContainerOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CreateSharedPackageContainerOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Members(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Members)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    pub fn ForceAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateCollisionOption(&self) -> ::windows_core::Result<SharedPackageContainerCreationCollisionOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SharedPackageContainerCreationCollisionOptions>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCollisionOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerCreationCollisionOptions>(result__)
        }
    }
    pub fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCreateCollisionOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for CreateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for CreateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateSharedPackageContainerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerOptions;{c2ab6ece-f664-5c8e-a4b3-2a33276d3dde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = <ICreateSharedPackageContainerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerOptions";
}
impl ::core::convert::From<CreateSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: CreateSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: &CreateSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerOptions {}
#[repr(transparent)]
pub struct CreateSharedPackageContainerResult(::windows_core::IUnknown);
impl CreateSharedPackageContainerResult {
    pub fn Container(&self) -> ::windows_core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SharedPackageContainerOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerResult {}
impl ::core::fmt::Debug for CreateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CreateSharedPackageContainerResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.CreateSharedPackageContainerResult;{ce8810bf-151c-5707-b936-497e564afc7a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = <ICreateSharedPackageContainerResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerResult";
}
impl ::core::convert::From<CreateSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: CreateSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: &CreateSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CreateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for CreateSharedPackageContainerResult {}
#[repr(transparent)]
pub struct DeleteSharedPackageContainerOptions(::windows_core::IUnknown);
impl DeleteSharedPackageContainerOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeleteSharedPackageContainerOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ForceAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllUsers(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllUsers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllUsers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllUsers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for DeleteSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerOptions {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeleteSharedPackageContainerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerOptions;{9d81865f-986e-5138-8b5d-384d8e66ed6c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = <IDeleteSharedPackageContainerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerOptions";
}
impl ::core::convert::From<DeleteSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeleteSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: DeleteSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: &DeleteSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeleteSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerOptions {}
#[repr(transparent)]
pub struct DeleteSharedPackageContainerResult(::windows_core::IUnknown);
impl DeleteSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SharedPackageContainerOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for DeleteSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerResult {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeleteSharedPackageContainerResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeleteSharedPackageContainerResult;{35398884-5736-517b-85bc-e598c81ab284})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = <IDeleteSharedPackageContainerResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerResult";
}
impl ::core::convert::From<DeleteSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeleteSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: DeleteSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeleteSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: &DeleteSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeleteSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeleteSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for DeleteSharedPackageContainerResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: Self = Self(0u32);
    pub const ForceApplicationShutdown: Self = Self(1u32);
    pub const DevelopmentMode: Self = Self(2u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetApplicationShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const ForceUpdateFromAnyVersion: Self = Self(262144u32);
    pub const RetainFilesOnFailure: Self = Self(2097152u32);
    pub const StageInPlace: Self = Self(4194304u32);
}
impl ::core::marker::Copy for DeploymentOptions {}
impl ::core::clone::Clone for DeploymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeploymentOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeploymentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for DeploymentOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl ::core::marker::Copy for DeploymentProgress {}
impl ::core::clone::Clone for DeploymentProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DeploymentProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DeploymentProgress").field("state", &self.state).field("percentage", &self.percentage).finish()
    }
}
unsafe impl ::windows_core::Abi for DeploymentProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for DeploymentProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Management.Deployment.DeploymentProgress;enum(Windows.Management.Deployment.DeploymentProgressState;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DeploymentProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DeploymentProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for DeploymentProgress {}
impl ::core::default::Default for DeploymentProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
}
impl ::core::marker::Copy for DeploymentProgressState {}
impl ::core::clone::Clone for DeploymentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeploymentProgressState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeploymentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentProgressState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeploymentProgressState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentProgressState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeploymentResult(::windows_core::IUnknown);
impl DeploymentResult {
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn IsRegistered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDeploymentResult2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRegistered)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for DeploymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeploymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentResult {}
impl ::core::fmt::Debug for DeploymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeploymentResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.DeploymentResult;{2563b9ae-b77d-4c1f-8a7b-20e6ad515ef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
    const IID: ::windows_core::GUID = <IDeploymentResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeploymentResult";
}
impl ::core::convert::From<DeploymentResult> for ::windows_core::IUnknown {
    fn from(value: DeploymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows_core::IUnknown {
    fn from(value: &DeploymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeploymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeploymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeploymentResult> for ::windows_core::IInspectable {
    fn from(value: DeploymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows_core::IInspectable {
    fn from(value: &DeploymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeploymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeploymentResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[repr(transparent)]
pub struct FindSharedPackageContainerOptions(::windows_core::IUnknown);
impl FindSharedPackageContainerOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FindSharedPackageContainerOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageFamilyName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FindSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindSharedPackageContainerOptions {}
impl ::core::fmt::Debug for FindSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindSharedPackageContainerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FindSharedPackageContainerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.FindSharedPackageContainerOptions;{b40fc8fe-8384-54cc-817d-ae09d3b6a606})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFindSharedPackageContainerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.FindSharedPackageContainerOptions";
}
impl ::core::convert::From<FindSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: FindSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: &FindSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FindSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for FindSharedPackageContainerOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPackageOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddPackageOptions {
    type Vtable = IAddPackageOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05cee018_f68f_422b_95a4_66679ec77fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RelatedPackageUris: usize,
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows_core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RetainFilesOnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRetainFilesOnFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallerManager {
    type Vtable = IAppInstallerManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7ee21c3_2103_53ee_9b18_68afeab0033d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetAutoUpdateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, appinstallerinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClearAutoUpdateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PauseAutoUpdatesUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, datetime: ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallerManagerStatics {
    type Vtable = IAppInstallerManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc95a6ed5_fc59_5336_9b2e_2b07c5e61434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutoUpdateSettingsOptions {
    type Vtable = IAutoUpdateSettingsOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67491d87_35e1_512a_8968_1ae88d1be6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_applicationmodel::PackageVersion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Version: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_applicationmodel::PackageVersion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    SetVersion: usize,
    pub AppInstallerUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAppInstallerUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetOnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub UpdateUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdateUris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RepairUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RepairUris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageUris: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoUpdateSettingsOptionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutoUpdateSettingsOptionsStatics {
    type Vtable = IAutoUpdateSettingsOptionsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x887b337d_0c05_54d0_bd49_3bb7a2c084cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub CreateFromAppInstallerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    CreateFromAppInstallerInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateSharedPackageContainerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateSharedPackageContainerOptions {
    type Vtable = ICreateSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2ab6ece_f664_5c8e_a4b3_2a33276d3dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Members: usize,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CreateCollisionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows_core::HRESULT,
    pub SetCreateCollisionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SharedPackageContainerCreationCollisionOptions) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateSharedPackageContainerResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreateSharedPackageContainerResult {
    type Vtable = ICreateSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce8810bf_151c_5707_b936_497e564afc7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeleteSharedPackageContainerOptions {
    type Vtable = IDeleteSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d81865f_986e_5138_8b5d_384d8e66ed6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeleteSharedPackageContainerResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeleteSharedPackageContainerResult {
    type Vtable = IDeleteSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35398884_5736_517b_85bc_e598c81ab284);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2563b9ae_b77d_4c1f_8a7b_20e6ad515ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentResult2 {
    type Vtable = IDeploymentResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc0e715c_5a01_4bd7_bcf1_381c8c82e04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsRegistered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindSharedPackageContainerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindSharedPackageContainerOptions {
    type Vtable = IFindSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb40fc8fe_8384_54cc_817d_ae09d3b6a606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageAllUserProvisioningOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda35aa22_1de0_5d3e_99ff_d24f3118bf5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ProjectionOrderPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProjectionOrderPackageFamilyNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager {
    type Vtable = IPackageManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddPackageAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub UpdatePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    UpdatePackageAsync: usize,
    pub RemovePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StagePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StagePackageAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterPackageAsync: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackages: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindUsers: usize,
    pub SetPackageState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagestate: PackageState) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-applicationmodel")]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    FindPackageByPackageFullName: usize,
    pub CleanupPackageForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager10 {
    type Vtable = IPackageManager10_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7d7d07e_2e66_4093_aed5_e093ed87b3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager10_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProvisionPackageForAllUsersWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager2 {
    type Vtable = IPackageManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7aad08d_0840_46f2_b5d8_cad47693a095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemovePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, removaloptions: RemovalOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StagePackageWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StagePackageWithOptionsAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterPackageByFullNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, dependencypackagefullnames: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterPackageByFullNameAsync: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: usize,
    pub StageUserDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager3 {
    type Vtable = IPackageManager3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaad9948_36f1_41a7_9188_bc263e0dcb72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddPackageVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagestorepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AddPackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddPackageToVolumeAsync: usize,
    pub ClearPackageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, status: PackageStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterPackageWithAppDataVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterPackageWithAppDataVolumeAsync: usize,
    pub FindPackageVolumeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindPackageVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindPackageVolumes: usize,
    pub GetDefaultPackageVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MovePackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemovePackageVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultPackageVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPackageStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, status: PackageStatus) -> ::windows_core::HRESULT,
    pub SetPackageVolumeOfflineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagevolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPackageVolumeOnlineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagevolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub StagePackageToVolumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StagePackageToVolumeAsync: usize,
    pub StageUserDataWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager4 {
    type Vtable = IPackageManager4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c719963_bab6_46bf_8ff7_da4719230ae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetPackageVolumesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetPackageVolumesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager5 {
    type Vtable = IPackageManager5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x711f3117_1afd_4313_978c_9bb6e1b864a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub AddPackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, externalpackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddPackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub StagePackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, externalpackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StagePackageToVolumeAndOptionalPackagesAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterPackageByFamilyNameAndOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, dependencypackagefamilynames: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterPackageByFamilyNameAndOptionalPackagesAsync: usize,
    pub DebugSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager6 {
    type Vtable = IPackageManager6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0847e909_53cd_4e4f_832e_57d180f6e447);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProvisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows_core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAddPackageByAppInstallerFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows_core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AddPackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, options: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, packageuristoinstall: ::windows_core::RawPtr, relatedpackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddPackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub StagePackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, options: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, packageuristoinstall: ::windows_core::RawPtr, relatedpackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    StagePackageToVolumeAndRelatedSetAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RequestAddPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, relatedpackageuris: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestAddPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager7 {
    type Vtable = IPackageManager7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf28654f4_2ba7_4b80_88d6_be15f9a23fba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub RequestAddPackageAndRelatedSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, dependencypackageuris: ::windows_core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows_core::RawPtr, optionalpackagefamilynames: ::windows_core::RawPtr, relatedpackageuris: ::windows_core::RawPtr, packageuristoinstall: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestAddPackageAndRelatedSetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager8 {
    type Vtable = IPackageManager8_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8575330_1298_4ee2_80ee_7f659c5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager8_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeprovisionPackageForAllUsersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManager9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManager9 {
    type Vtable = IPackageManager9_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1aa79035_cc71_4b2e_80a6_c7041d8579a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager9_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindProvisionedPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindProvisionedPackages: usize,
    pub AddPackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StagePackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageuri: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RegisterPackageByUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifesturi: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterPackagesByFullNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullnames: ::windows_core::RawPtr, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterPackagesByFullNameAsync: usize,
    pub SetPackageStubPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, usestub: PackageStubPreference) -> ::windows_core::HRESULT,
    pub GetPackageStubPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut PackageStubPreference) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageManagerDebugSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a611683_a988_4fcf_8f0f_ce175898e8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub SetContentGroupStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows_core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, state: ::winrt_applicationmodel::PackageContentGroupState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    SetContentGroupStateAsync: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub SetContentGroupStateWithPercentageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: ::windows_core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, state: ::winrt_applicationmodel::PackageContentGroupState, completionpercentage: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    SetContentGroupStateWithPercentageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUserInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageUserInformation {
    type Vtable = IPackageUserInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6383423_fa09_4cbc_9055_15ca275e2e7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUserInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageInstallState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageVolume(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageVolume {
    type Vtable = IPackageVolume_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf2672c3_1a40_4450_9739_2ace2e898853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSystemVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MountPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageStorePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportsHardLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackages: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByNamePublisher: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByNamePublisherWithPackagesTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByNamePublisherWithPackagesTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackageByPackageFullName: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: usize,
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-applicationmodel", feature = "winrt-foundation")))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageVolume2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageVolume2 {
    type Vtable = IPackageVolume2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46abcf2e_9dd4_47a2_ab8c_c6408349bcd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsFullTrustPackageSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAppxInstallSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetAvailableSpaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRegisterPackageOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x677112a7_50d4_496c_8415_0602b4c6d3bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DependencyPackageUris: usize,
    pub AppDataVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAppDataVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageFamilyNames: usize,
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedPackageContainer {
    type Vtable = ISharedPackageContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x177f1aa9_151e_5ef7_b1d9_2fba0b4b0d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetMembers: usize,
    pub RemovePackageFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ResetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe353068_1ef7_5ac8_ab3f_0b9f612f0274);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindContainers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindContainers: usize,
    #[cfg(feature = "winrt-foundation")]
    pub FindContainersWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindContainersWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedPackageContainerManagerStatics {
    type Vtable = ISharedPackageContainerManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ef56348_838a_5f55_a89e_1198a2c627e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForProvisioning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerMember(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe0d0438_43c9_5426_b89c_f79bf85ddff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedPackageContainerMemberFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISharedPackageContainerMemberFactory {
    type Vtable = ISharedPackageContainerMemberFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49b0ceeb_498f_5a62_b738_b3ca0d436704);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStagePackageOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStagePackageOptions {
    type Vtable = IStagePackageOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b110c9c_b95d_4c56_bd36_6d656800d06b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DependencyPackageUris: usize,
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageFamilyNames: usize,
    #[cfg(feature = "winrt-foundation")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    OptionalPackageUris: usize,
    #[cfg(feature = "winrt-foundation")]
    pub RelatedPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RelatedPackageUris: usize,
    pub ExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows_core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80672e83_7194_59f9_b5b9_daa5375f130a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RequirePackagesPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequirePackagesPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUpdateSharedPackageContainerResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa407df7_c72d_5458_aea3_4645b6a8ee99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PackageAllUserProvisioningOptions(::windows_core::IUnknown);
impl PackageAllUserProvisioningOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PackageAllUserProvisioningOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProjectionOrderPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionOrderPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageAllUserProvisioningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageAllUserProvisioningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageAllUserProvisioningOptions {}
impl ::core::fmt::Debug for PackageAllUserProvisioningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageAllUserProvisioningOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageAllUserProvisioningOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageAllUserProvisioningOptions;{da35aa22-1de0-5d3e-99ff-d24f3118bf5e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageAllUserProvisioningOptions {
    type Vtable = IPackageAllUserProvisioningOptions_Vtbl;
    const IID: ::windows_core::GUID = <IPackageAllUserProvisioningOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.PackageAllUserProvisioningOptions";
}
impl ::core::convert::From<PackageAllUserProvisioningOptions> for ::windows_core::IUnknown {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageAllUserProvisioningOptions> for ::windows_core::IUnknown {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageAllUserProvisioningOptions> for ::windows_core::IInspectable {
    fn from(value: PackageAllUserProvisioningOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageAllUserProvisioningOptions> for ::windows_core::IInspectable {
    fn from(value: &PackageAllUserProvisioningOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageAllUserProvisioningOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageAllUserProvisioningOptions {}
unsafe impl ::core::marker::Sync for PackageAllUserProvisioningOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: Self = Self(0i32);
    pub const Staged: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
    pub const Paused: Self = Self(6i32);
}
impl ::core::marker::Copy for PackageInstallState {}
impl ::core::clone::Clone for PackageInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageInstallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageInstallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageInstallState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageInstallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PackageManager(::windows_core::IUnknown);
impl PackageManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PackageManager, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn UpdatePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdatePackageAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn RemovePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StagePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageAsync)(::windows_core::Interface::as_raw(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityId)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindUsers<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<PackageUserInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindUsers)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<PackageUserInformation>>(result__)
        }
    }
    pub fn SetPackageState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0, packagestate: PackageState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageState)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), packagestate).ok() }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<::winrt_applicationmodel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageByPackageFullName)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Package>(result__)
        }
    }
    pub fn CleanupPackageForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagename: Param0, usersecurityid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CleanupPackageForUserAsync)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), usersecurityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByPackageFamilyName)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows_core::Result<::winrt_applicationmodel::Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Package>(result__)
        }
    }
    pub fn ProvisionPackageForAllUsersWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, PackageAllUserProvisioningOptions>>(&self, mainpackagefamilyname: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager10>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionPackageForAllUsersWithOptionsAsync)(::windows_core::Interface::as_raw(this), mainpackagefamilyname.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn RemovePackageWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0, removaloptions: RemovalOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageWithOptionsAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), removaloptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StagePackageWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageWithOptionsAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterPackageByFullNameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, mainpackagefullname: Param0, dependencypackagefullnames: Param1, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageByFullNameAsync)(::windows_core::Interface::as_raw(this), mainpackagefullname.into_param().abi(), dependencypackagefullnames.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesWithPackageTypes)(::windows_core::Interface::as_raw(this), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByNamePublisherWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisherWithPackageTypes)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::winrt_applicationmodel::Package>>(result__)
        }
    }
    pub fn StageUserDataAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StageUserDataAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn AddPackageVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagestorepath: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageVolume>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageVolumeAsync)(::windows_core::Interface::as_raw(this), packagestorepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageVolume>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddPackageToVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, targetvolume: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageToVolumeAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn ClearPackageStatus<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ClearPackageStatus)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), status).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterPackageWithAppDataVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>>(&self, manifesturi: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, appdatavolume: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageWithAppDataVolumeAsync)(::windows_core::Interface::as_raw(this), manifesturi.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn FindPackageVolumeByName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, volumename: Param0) -> ::windows_core::Result<PackageVolume> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageVolumeByName)(::windows_core::Interface::as_raw(this), volumename.into_param().abi(), result__.as_mut_ptr()).from_abi::<PackageVolume>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindPackageVolumes(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<PackageVolume>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageVolumes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<PackageVolume>>(result__)
        }
    }
    pub fn GetDefaultPackageVolume(&self) -> ::windows_core::Result<PackageVolume> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultPackageVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn MovePackageToVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, PackageVolume>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions, targetvolume: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MovePackageToVolumeAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn RemovePackageVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageVolumeAsync)(::windows_core::Interface::as_raw(this), volume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn SetDefaultPackageVolume<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, volume: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultPackageVolume)(::windows_core::Interface::as_raw(this), volume.into_param().abi()).ok() }
    }
    pub fn SetPackageStatus<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0, status: PackageStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageStatus)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), status).ok() }
    }
    pub fn SetPackageVolumeOfflineAsync<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPackageVolumeOfflineAsync)(::windows_core::Interface::as_raw(this), packagevolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn SetPackageVolumeOnlineAsync<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, packagevolume: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetPackageVolumeOnlineAsync)(::windows_core::Interface::as_raw(this), packagevolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StagePackageToVolumeAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, targetvolume: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageToVolumeAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn StageUserDataWithOptionsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0, deploymentoptions: DeploymentOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StageUserDataWithOptionsAsync)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), deploymentoptions, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetPackageVolumesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PackageVolume>>> {
        let this = &::windows_core::Interface::cast::<IPackageManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPackageVolumesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<PackageVolume>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddPackageToVolumeAndOptionalPackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, targetvolume: Param3, optionalpackagefamilynames: Param4, externalpackageuris: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageToVolumeAndOptionalPackagesAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StagePackageToVolumeAndOptionalPackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, targetvolume: Param3, optionalpackagefamilynames: Param4, externalpackageuris: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageToVolumeAndOptionalPackagesAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), externalpackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, mainpackagefamilyname: Param0, dependencypackagefamilynames: Param1, deploymentoptions: DeploymentOptions, appdatavolume: Param3, optionalpackagefamilynames: Param4) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageByFamilyNameAndOptionalPackagesAsync)(::windows_core::Interface::as_raw(this), mainpackagefamilyname.into_param().abi(), dependencypackagefamilynames.into_param().abi(), deploymentoptions, appdatavolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn DebugSettings(&self) -> ::windows_core::Result<PackageManagerDebugSettings> {
        let this = &::windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DebugSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageManagerDebugSettings>(result__)
        }
    }
    pub fn ProvisionPackageForAllUsersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionPackageForAllUsersAsync)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn AddPackageByAppInstallerFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageByAppInstallerFileAsync)(::windows_core::Interface::as_raw(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn RequestAddPackageByAppInstallerFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param2: ::windows_core::IntoParam<'a, PackageVolume>>(&self, appinstallerfileuri: Param0, options: AddPackageByAppInstallerOptions, targetvolume: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAddPackageByAppInstallerFileAsync)(::windows_core::Interface::as_raw(this), appinstallerfileuri.into_param().abi(), options, targetvolume.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddPackageToVolumeAndRelatedSetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param6: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageToVolumeAndRelatedSetAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn StagePackageToVolumeAndRelatedSetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param6: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        options: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        packageuristoinstall: Param5,
        relatedpackageuris: Param6,
    ) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageToVolumeAndRelatedSetAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), options, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), packageuristoinstall.into_param().abi(), relatedpackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestAddPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, packageuri: Param0, dependencypackageuris: Param1, deploymentoptions: DeploymentOptions, targetvolume: Param3, optionalpackagefamilynames: Param4, relatedpackageuris: Param5) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAddPackageAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestAddPackageAndRelatedSetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param3: ::windows_core::IntoParam<'a, PackageVolume>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param5: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param6: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(
        &self,
        packageuri: Param0,
        dependencypackageuris: Param1,
        deploymentoptions: DeploymentOptions,
        targetvolume: Param3,
        optionalpackagefamilynames: Param4,
        relatedpackageuris: Param5,
        packageuristoinstall: Param6,
    ) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAddPackageAndRelatedSetAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), dependencypackageuris.into_param().abi(), deploymentoptions, targetvolume.into_param().abi(), optionalpackagefamilynames.into_param().abi(), relatedpackageuris.into_param().abi(), packageuristoinstall.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn DeprovisionPackageForAllUsersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeprovisionPackageForAllUsersAsync)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindProvisionedPackages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindProvisionedPackages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    pub fn AddPackageByUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, AddPackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddPackageByUriAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn StagePackageByUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, StagePackageOptions>>(&self, packageuri: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StagePackageByUriAsync)(::windows_core::Interface::as_raw(this), packageuri.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn RegisterPackageByUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, RegisterPackageOptions>>(&self, manifesturi: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackageByUriAsync)(::windows_core::Interface::as_raw(this), manifesturi.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterPackagesByFullNameAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, RegisterPackageOptions>>(&self, packagefullnames: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPackagesByFullNameAsync)(::windows_core::Interface::as_raw(this), packagefullnames.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>(result__)
        }
    }
    pub fn SetPackageStubPreference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0, usestub: PackageStubPreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPackageStubPreference)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), usestub).ok() }
    }
    pub fn GetPackageStubPreference<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<PackageStubPreference> {
        let this = &::windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageStubPreference>::zeroed();
            (::windows_core::Interface::vtable(this).GetPackageStubPreference)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<PackageStubPreference>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManager {}
impl ::core::fmt::Debug for PackageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManager;{9a7d4b65-5e8f-4fc7-a2e5-7f6925cb8b53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageManager {
    type Vtable = IPackageManager_Vtbl;
    const IID: ::windows_core::GUID = <IPackageManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
impl ::core::convert::From<PackageManager> for ::windows_core::IUnknown {
    fn from(value: PackageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageManager> for ::windows_core::IUnknown {
    fn from(value: &PackageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageManager> for ::windows_core::IInspectable {
    fn from(value: PackageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageManager> for ::windows_core::IInspectable {
    fn from(value: &PackageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageManager {}
unsafe impl ::core::marker::Sync for PackageManager {}
#[repr(transparent)]
pub struct PackageManagerDebugSettings(::windows_core::IUnknown);
impl PackageManagerDebugSettings {
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn SetContentGroupStateAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: ::winrt_applicationmodel::PackageContentGroupState) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetContentGroupStateAsync)(::windows_core::Interface::as_raw(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn SetContentGroupStateWithPercentageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Package>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, package: Param0, contentgroupname: Param1, state: ::winrt_applicationmodel::PackageContentGroupState, completionpercentage: f64) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetContentGroupStateWithPercentageAsync)(::windows_core::Interface::as_raw(this), package.into_param().abi(), contentgroupname.into_param().abi(), state, completionpercentage, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageManagerDebugSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageManagerDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManagerDebugSettings {}
impl ::core::fmt::Debug for PackageManagerDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManagerDebugSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageManagerDebugSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageManagerDebugSettings;{1a611683-a988-4fcf-8f0f-ce175898e8eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageManagerDebugSettings {
    type Vtable = IPackageManagerDebugSettings_Vtbl;
    const IID: ::windows_core::GUID = <IPackageManagerDebugSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManagerDebugSettings";
}
impl ::core::convert::From<PackageManagerDebugSettings> for ::windows_core::IUnknown {
    fn from(value: PackageManagerDebugSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageManagerDebugSettings> for ::windows_core::IUnknown {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageManagerDebugSettings> for ::windows_core::IInspectable {
    fn from(value: PackageManagerDebugSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageManagerDebugSettings> for ::windows_core::IInspectable {
    fn from(value: &PackageManagerDebugSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageManagerDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageManagerDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageManagerDebugSettings {}
unsafe impl ::core::marker::Sync for PackageManagerDebugSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: Self = Self(0i32);
    pub const LicenseInvalid: Self = Self(1i32);
    pub const Modified: Self = Self(2i32);
    pub const Tampered: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageState {}
impl ::core::clone::Clone for PackageState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: Self = Self(0u32);
    pub const LicenseIssue: Self = Self(1u32);
    pub const Modified: Self = Self(2u32);
    pub const Tampered: Self = Self(4u32);
    pub const Disabled: Self = Self(8u32);
}
impl ::core::marker::Copy for PackageStatus {}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatus").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStatus;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: Self = Self(0i32);
    pub const Stub: Self = Self(1i32);
}
impl ::core::marker::Copy for PackageStubPreference {}
impl ::core::clone::Clone for PackageStubPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageStubPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageStubPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageStubPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStubPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageStubPreference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStubPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: Self = Self(0u32);
    pub const Main: Self = Self(1u32);
    pub const Framework: Self = Self(2u32);
    pub const Resource: Self = Self(4u32);
    pub const Bundle: Self = Self(8u32);
    pub const Xap: Self = Self(16u32);
    pub const Optional: Self = Self(32u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PackageTypes {}
impl ::core::clone::Clone for PackageTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for PackageTypes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PackageUserInformation(::windows_core::IUnknown);
impl PackageUserInformation {
    pub fn UserSecurityId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserSecurityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn InstallState(&self) -> ::windows_core::Result<PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageInstallState>::zeroed();
            (::windows_core::Interface::vtable(this).InstallState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageInstallState>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUserInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUserInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUserInformation {}
impl ::core::fmt::Debug for PackageUserInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUserInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageUserInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageUserInformation;{f6383423-fa09-4cbc-9055-15ca275e2e7e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageUserInformation {
    type Vtable = IPackageUserInformation_Vtbl;
    const IID: ::windows_core::GUID = <IPackageUserInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.PackageUserInformation";
}
impl ::core::convert::From<PackageUserInformation> for ::windows_core::IUnknown {
    fn from(value: PackageUserInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUserInformation> for ::windows_core::IUnknown {
    fn from(value: &PackageUserInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageUserInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUserInformation> for ::windows_core::IInspectable {
    fn from(value: PackageUserInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUserInformation> for ::windows_core::IInspectable {
    fn from(value: &PackageUserInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageUserInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageUserInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUserInformation {}
unsafe impl ::core::marker::Sync for PackageUserInformation {}
#[repr(transparent)]
pub struct PackageVolume(::windows_core::IUnknown);
impl PackageVolume {
    pub fn IsOffline(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOffline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsSystemVolume(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSystemVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MountPoint(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MountPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PackageStorePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageStorePath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SupportsHardLinks(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SupportsHardLinks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackages(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackages)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByNamePublisher<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagename: Param0, packagepublisher: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(::windows_core::Interface::as_raw(this), packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByPackageFamilyName)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesWithPackageTypes)(::windows_core::Interface::as_raw(this), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByNamePublisherWithPackagesTypes<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagetypes: PackageTypes, packagename: Param1, packagepublisher: Param2) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByNamePublisherWithPackagesTypes)(::windows_core::Interface::as_raw(this), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagetypes: PackageTypes, packagefamilyname: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(::windows_core::Interface::as_raw(this), packagetypes, packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackageByPackageFullName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, packagefullname: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageByPackageFullName)(::windows_core::Interface::as_raw(this), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityId)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdNamePublisher<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagename: Param1, packagepublisher: Param2) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagefamilyname: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagetypes, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes, packagename: Param2, packagepublisher: Param3) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagetypes, packagename.into_param().abi(), packagepublisher.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagetypes: PackageTypes, packagefamilyname: Param2) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagetypes, packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-applicationmodel", feature = "winrt-foundation"))]
    pub fn FindPackageByUserSecurityIdPackageFullName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, usersecurityid: Param0, packagefullname: Param1) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(::windows_core::Interface::as_raw(this), usersecurityid.into_param().abi(), packagefullname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_applicationmodel::Package>>(result__)
        }
    }
    pub fn IsFullTrustPackageSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFullTrustPackageSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAppxInstallSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppxInstallSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetAvailableSpaceAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<u64>> {
        let this = &::windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableSpaceAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<u64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageVolume {}
impl ::core::fmt::Debug for PackageVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageVolume {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.PackageVolume;{cf2672c3-1a40-4450-9739-2ace2e898853})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageVolume {
    type Vtable = IPackageVolume_Vtbl;
    const IID: ::windows_core::GUID = <IPackageVolume as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.PackageVolume";
}
impl ::core::convert::From<PackageVolume> for ::windows_core::IUnknown {
    fn from(value: PackageVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageVolume> for ::windows_core::IUnknown {
    fn from(value: &PackageVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageVolume> for ::windows_core::IInspectable {
    fn from(value: PackageVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageVolume> for ::windows_core::IInspectable {
    fn from(value: &PackageVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageVolume {}
unsafe impl ::core::marker::Sync for PackageVolume {}
#[repr(transparent)]
pub struct RegisterPackageOptions(::windows_core::IUnknown);
impl RegisterPackageOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RegisterPackageOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DependencyPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn AppDataVolume(&self) -> ::windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppDataVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetAppDataVolume<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppDataVolume)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn ExternalLocationUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExternalLocationUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetExternalLocationUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExternalLocationUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeveloperMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeveloperMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceTargetAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceTargetAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InstallAllResources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallAllResources)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StageInPlace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStageInPlace)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowUnsigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowUnsigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for RegisterPackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RegisterPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegisterPackageOptions {}
impl ::core::fmt::Debug for RegisterPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegisterPackageOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RegisterPackageOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.RegisterPackageOptions;{677112a7-50d4-496c-8415-0602b4c6d3bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RegisterPackageOptions {
    type Vtable = IRegisterPackageOptions_Vtbl;
    const IID: ::windows_core::GUID = <IRegisterPackageOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RegisterPackageOptions";
}
impl ::core::convert::From<RegisterPackageOptions> for ::windows_core::IUnknown {
    fn from(value: RegisterPackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegisterPackageOptions> for ::windows_core::IUnknown {
    fn from(value: &RegisterPackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RegisterPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RegisterPackageOptions> for ::windows_core::IInspectable {
    fn from(value: RegisterPackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RegisterPackageOptions> for ::windows_core::IInspectable {
    fn from(value: &RegisterPackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RegisterPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RegisterPackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RegisterPackageOptions {}
unsafe impl ::core::marker::Sync for RegisterPackageOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: Self = Self(0u32);
    pub const PreserveApplicationData: Self = Self(4096u32);
    pub const PreserveRoamableApplicationData: Self = Self(128u32);
    pub const RemoveForAllUsers: Self = Self(524288u32);
}
impl ::core::marker::Copy for RemovalOptions {}
impl ::core::clone::Clone for RemovalOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemovalOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RemovalOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemovalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemovalOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for RemovalOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.RemovalOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SharedPackageContainer(::windows_core::IUnknown);
impl SharedPackageContainer {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMembers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetMembers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SharedPackageContainerMember>>(result__)
        }
    }
    pub fn RemovePackageFamily<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, UpdateSharedPackageContainerOptions>>(&self, packagefamilyname: Param0, options: Param1) -> ::windows_core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemovePackageFamily)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
    pub fn ResetData(&self) -> ::windows_core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ResetData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UpdateSharedPackageContainerResult>(result__)
        }
    }
}
impl ::core::clone::Clone for SharedPackageContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainer {}
impl ::core::fmt::Debug for SharedPackageContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SharedPackageContainer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainer;{177f1aa9-151e-5ef7-b1d9-2fba0b4b0d17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SharedPackageContainer {
    type Vtable = ISharedPackageContainer_Vtbl;
    const IID: ::windows_core::GUID = <ISharedPackageContainer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainer";
}
impl ::core::convert::From<SharedPackageContainer> for ::windows_core::IUnknown {
    fn from(value: SharedPackageContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainer> for ::windows_core::IUnknown {
    fn from(value: &SharedPackageContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SharedPackageContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SharedPackageContainer> for ::windows_core::IInspectable {
    fn from(value: SharedPackageContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainer> for ::windows_core::IInspectable {
    fn from(value: &SharedPackageContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SharedPackageContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SharedPackageContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainer {}
unsafe impl ::core::marker::Sync for SharedPackageContainer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: Self = Self(0i32);
    pub const MergeWithExisting: Self = Self(1i32);
    pub const ReplaceExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for SharedPackageContainerCreationCollisionOptions {}
impl ::core::clone::Clone for SharedPackageContainerCreationCollisionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SharedPackageContainerCreationCollisionOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SharedPackageContainerCreationCollisionOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for SharedPackageContainerCreationCollisionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerCreationCollisionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SharedPackageContainerCreationCollisionOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerCreationCollisionOptions;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SharedPackageContainerManager(::windows_core::IUnknown);
impl SharedPackageContainerManager {
    pub fn CreateContainer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, CreateSharedPackageContainerOptions>>(&self, name: Param0, options: Param1) -> ::windows_core::Result<CreateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateContainer)(::windows_core::Interface::as_raw(this), name.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<CreateSharedPackageContainerResult>(result__)
        }
    }
    pub fn DeleteContainer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, DeleteSharedPackageContainerOptions>>(&self, id: Param0, options: Param1) -> ::windows_core::Result<DeleteSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteContainer)(::windows_core::Interface::as_raw(this), id.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeleteSharedPackageContainerResult>(result__)
        }
    }
    pub fn GetContainer<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContainer)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<SharedPackageContainer>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindContainers(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindContainers)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindContainersWithOptions<'a, Param0: ::windows_core::IntoParam<'a, FindSharedPackageContainerOptions>>(&self, options: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindContainersWithOptions)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<SharedPackageContainer>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(usersid: Param0) -> ::windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), usersid.into_param().abi(), result__.as_mut_ptr()).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn GetForProvisioning() -> ::windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForProvisioning)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerManager>(result__)
        })
    }
    pub fn ISharedPackageContainerManagerStatics<R, F: FnOnce(&ISharedPackageContainerManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SharedPackageContainerManager, ISharedPackageContainerManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SharedPackageContainerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerManager {}
impl ::core::fmt::Debug for SharedPackageContainerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SharedPackageContainerManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerManager;{be353068-1ef7-5ac8-ab3f-0b9f612f0274})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SharedPackageContainerManager {
    type Vtable = ISharedPackageContainerManager_Vtbl;
    const IID: ::windows_core::GUID = <ISharedPackageContainerManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerManager";
}
impl ::core::convert::From<SharedPackageContainerManager> for ::windows_core::IUnknown {
    fn from(value: SharedPackageContainerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainerManager> for ::windows_core::IUnknown {
    fn from(value: &SharedPackageContainerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SharedPackageContainerManager> for ::windows_core::IInspectable {
    fn from(value: SharedPackageContainerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainerManager> for ::windows_core::IInspectable {
    fn from(value: &SharedPackageContainerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SharedPackageContainerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SharedPackageContainerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainerManager {}
unsafe impl ::core::marker::Sync for SharedPackageContainerManager {}
#[repr(transparent)]
pub struct SharedPackageContainerMember(::windows_core::IUnknown);
impl SharedPackageContainerMember {
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(packagefamilyname: Param0) -> ::windows_core::Result<SharedPackageContainerMember> {
        Self::ISharedPackageContainerMemberFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), packagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<SharedPackageContainerMember>(result__)
        })
    }
    pub fn ISharedPackageContainerMemberFactory<R, F: FnOnce(&ISharedPackageContainerMemberFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SharedPackageContainerMember, ISharedPackageContainerMemberFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SharedPackageContainerMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerMember {}
impl ::core::fmt::Debug for SharedPackageContainerMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerMember").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SharedPackageContainerMember {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.SharedPackageContainerMember;{fe0d0438-43c9-5426-b89c-f79bf85ddff4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SharedPackageContainerMember {
    type Vtable = ISharedPackageContainerMember_Vtbl;
    const IID: ::windows_core::GUID = <ISharedPackageContainerMember as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerMember";
}
impl ::core::convert::From<SharedPackageContainerMember> for ::windows_core::IUnknown {
    fn from(value: SharedPackageContainerMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainerMember> for ::windows_core::IUnknown {
    fn from(value: &SharedPackageContainerMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SharedPackageContainerMember> for ::windows_core::IInspectable {
    fn from(value: SharedPackageContainerMember) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SharedPackageContainerMember> for ::windows_core::IInspectable {
    fn from(value: &SharedPackageContainerMember) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SharedPackageContainerMember {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SharedPackageContainerMember {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SharedPackageContainerMember {}
unsafe impl ::core::marker::Sync for SharedPackageContainerMember {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const BlockedByPolicy: Self = Self(1i32);
    pub const AlreadyExists: Self = Self(2i32);
    pub const PackageFamilyExistsInAnotherContainer: Self = Self(3i32);
    pub const NotFound: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for SharedPackageContainerOperationStatus {}
impl ::core::clone::Clone for SharedPackageContainerOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SharedPackageContainerOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SharedPackageContainerOperationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SharedPackageContainerOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerOperationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SharedPackageContainerOperationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerOperationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct StagePackageOptions(::windows_core::IUnknown);
impl StagePackageOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StagePackageOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DependencyPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn TargetVolume(&self) -> ::windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetVolume)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVolume>(result__)
        }
    }
    pub fn SetTargetVolume<'a, Param0: ::windows_core::IntoParam<'a, PackageVolume>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetVolume)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageFamilyNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn OptionalPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RelatedPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelatedPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn ExternalLocationUri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExternalLocationUri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetExternalLocationUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExternalLocationUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> ::windows_core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StubPackageOption>::zeroed();
            (::windows_core::Interface::vtable(this).StubPackageOption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StubPackageOption>(result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStubPackageOption)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeveloperMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeveloperMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InstallAllResources)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallAllResources)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredContentGroupOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiredContentGroupOnly)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StageInPlace)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStageInPlace)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowUnsigned)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowUnsigned)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for StagePackageOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StagePackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StagePackageOptions {}
impl ::core::fmt::Debug for StagePackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StagePackageOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StagePackageOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.StagePackageOptions;{0b110c9c-b95d-4c56-bd36-6d656800d06b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StagePackageOptions {
    type Vtable = IStagePackageOptions_Vtbl;
    const IID: ::windows_core::GUID = <IStagePackageOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.StagePackageOptions";
}
impl ::core::convert::From<StagePackageOptions> for ::windows_core::IUnknown {
    fn from(value: StagePackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StagePackageOptions> for ::windows_core::IUnknown {
    fn from(value: &StagePackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StagePackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StagePackageOptions> for ::windows_core::IInspectable {
    fn from(value: StagePackageOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StagePackageOptions> for ::windows_core::IInspectable {
    fn from(value: &StagePackageOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StagePackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StagePackageOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StagePackageOptions {}
unsafe impl ::core::marker::Sync for StagePackageOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: Self = Self(0i32);
    pub const InstallFull: Self = Self(1i32);
    pub const InstallStub: Self = Self(2i32);
    pub const UsePreference: Self = Self(3i32);
}
impl ::core::marker::Copy for StubPackageOption {}
impl ::core::clone::Clone for StubPackageOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StubPackageOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StubPackageOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for StubPackageOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StubPackageOption").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StubPackageOption {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.StubPackageOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UpdateSharedPackageContainerOptions(::windows_core::IUnknown);
impl UpdateSharedPackageContainerOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<UpdateSharedPackageContainerOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ForceAppShutdown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceAppShutdown)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceAppShutdown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequirePackagesPresent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequirePackagesPresent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRequirePackagesPresent(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequirePackagesPresent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for UpdateSharedPackageContainerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UpdateSharedPackageContainerOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerOptions;{80672e83-7194-59f9-b5b9-daa5375f130a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UpdateSharedPackageContainerOptions {
    type Vtable = IUpdateSharedPackageContainerOptions_Vtbl;
    const IID: ::windows_core::GUID = <IUpdateSharedPackageContainerOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerOptions";
}
impl ::core::convert::From<UpdateSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerOptions> for ::windows_core::IUnknown {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UpdateSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: UpdateSharedPackageContainerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerOptions> for ::windows_core::IInspectable {
    fn from(value: &UpdateSharedPackageContainerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UpdateSharedPackageContainerOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerOptions {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerOptions {}
#[repr(transparent)]
pub struct UpdateSharedPackageContainerResult(::windows_core::IUnknown);
impl UpdateSharedPackageContainerResult {
    pub fn Status(&self) -> ::windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SharedPackageContainerOperationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SharedPackageContainerOperationStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for UpdateSharedPackageContainerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerResult {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UpdateSharedPackageContainerResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.UpdateSharedPackageContainerResult;{aa407df7-c72d-5458-aea3-4645b6a8ee99})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UpdateSharedPackageContainerResult {
    type Vtable = IUpdateSharedPackageContainerResult_Vtbl;
    const IID: ::windows_core::GUID = <IUpdateSharedPackageContainerResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerResult";
}
impl ::core::convert::From<UpdateSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerResult> for ::windows_core::IUnknown {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UpdateSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: UpdateSharedPackageContainerResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UpdateSharedPackageContainerResult> for ::windows_core::IInspectable {
    fn from(value: &UpdateSharedPackageContainerResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UpdateSharedPackageContainerResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UpdateSharedPackageContainerResult {}
unsafe impl ::core::marker::Sync for UpdateSharedPackageContainerResult {}
