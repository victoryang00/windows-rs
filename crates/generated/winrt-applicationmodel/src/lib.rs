
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Activation")]
pub mod Activation;
#[cfg(feature = "AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "AppService")]
pub mod AppService;
#[cfg(feature = "Appointments")]
pub mod Appointments;
#[cfg(feature = "Background")]
pub mod Background;
#[cfg(feature = "Calls")]
pub mod Calls;
#[cfg(feature = "Chat")]
pub mod Chat;
#[cfg(feature = "CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "Contacts")]
pub mod Contacts;
#[cfg(feature = "ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "Core")]
pub mod Core;
#[cfg(feature = "DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "Email")]
pub mod Email;
#[cfg(feature = "ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "Holographic")]
pub mod Holographic;
#[cfg(feature = "LockScreen")]
pub mod LockScreen;
#[cfg(feature = "Payments")]
pub mod Payments;
#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Resources")]
pub mod Resources;
#[cfg(feature = "Search")]
pub mod Search;
#[cfg(feature = "SocialInfo")]
pub mod SocialInfo;
#[cfg(feature = "Store")]
pub mod Store;
#[cfg(feature = "UserActivities")]
pub mod UserActivities;
#[cfg(feature = "UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "Wallet")]
pub mod Wallet;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddResourcePackageOptions(pub u32);
impl AddResourcePackageOptions {
    pub const None: Self = Self(0u32);
    pub const ForceTargetAppShutdown: Self = Self(1u32);
    pub const ApplyUpdateIfAvailable: Self = Self(2u32);
}
impl ::core::marker::Copy for AddResourcePackageOptions {}
impl ::core::clone::Clone for AddResourcePackageOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddResourcePackageOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AddResourcePackageOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddResourcePackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddResourcePackageOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AddResourcePackageOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AddResourcePackageOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AddResourcePackageOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AddResourcePackageOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AddResourcePackageOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for AddResourcePackageOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AddResourcePackageOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppDisplayInfo(::windows_core::IUnknown);
impl AppDisplayInfo {
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
    #[cfg(feature = "Storage_Streams")]
    pub fn GetLogo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, size: Param0) -> ::windows_core::Result<::winrt_storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLogo)(::windows_core::Interface::as_raw(this), size.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
}
impl ::core::clone::Clone for AppDisplayInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDisplayInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDisplayInfo {}
impl ::core::fmt::Debug for AppDisplayInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDisplayInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppDisplayInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppDisplayInfo;{1aeb1103-e4d4-41aa-a4f6-c4a276e79eac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppDisplayInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppDisplayInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppDisplayInfo";
}
impl ::core::convert::From<AppDisplayInfo> for ::windows_core::IUnknown {
    fn from(value: AppDisplayInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDisplayInfo> for ::windows_core::IUnknown {
    fn from(value: &AppDisplayInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppDisplayInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppDisplayInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDisplayInfo> for ::windows_core::IInspectable {
    fn from(value: AppDisplayInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDisplayInfo> for ::windows_core::IInspectable {
    fn from(value: &AppDisplayInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppDisplayInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppDisplayInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDisplayInfo {}
unsafe impl ::core::marker::Sync for AppDisplayInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppExecutionContext(pub i32);
impl AppExecutionContext {
    pub const Unknown: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Guest: Self = Self(2i32);
}
impl ::core::marker::Copy for AppExecutionContext {}
impl ::core::clone::Clone for AppExecutionContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppExecutionContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppExecutionContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppExecutionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppExecutionContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppExecutionContext {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppExecutionContext;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppInfo(::windows_core::IUnknown);
impl AppInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayInfo(&self) -> ::windows_core::Result<AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppDisplayInfo>(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = &::windows_core::Interface::cast::<IAppInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn ExecutionContext(&self) -> ::windows_core::Result<AppExecutionContext> {
        let this = &::windows_core::Interface::cast::<IAppInfo3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppExecutionContext>::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppExecutionContext>(result__)
        }
    }
    pub fn SupportedFileExtensions(&self) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IAppInfo4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFileExtensions)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn Current() -> ::windows_core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInfo>(result__)
        })
    }
    pub fn GetFromAppUserModelId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appusermodelid: Param0) -> ::windows_core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromAppUserModelId)(::windows_core::Interface::as_raw(this), appusermodelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppInfo>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetFromAppUserModelIdForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, appusermodelid: Param1) -> ::windows_core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFromAppUserModelIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), appusermodelid.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppInfo>(result__)
        })
    }
    pub fn IAppInfoStatics<R, F: FnOnce(&IAppInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppInfo, IAppInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInfo {}
impl ::core::fmt::Debug for AppInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInfo;{cf7f59b3-6a09-4de8-a6c0-5792d56880d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInfo {
    type Vtable = IAppInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInfo";
}
impl ::core::convert::From<AppInfo> for ::windows_core::IUnknown {
    fn from(value: AppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInfo> for ::windows_core::IUnknown {
    fn from(value: &AppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInfo> for ::windows_core::IInspectable {
    fn from(value: AppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInfo> for ::windows_core::IInspectable {
    fn from(value: &AppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInfo {}
unsafe impl ::core::marker::Sync for AppInfo {}
#[repr(transparent)]
pub struct AppInstallerInfo(::windows_core::IUnknown);
impl AppInstallerInfo {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn OnLaunch(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).OnLaunch)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HoursBetweenUpdateChecks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ShowPrompt(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShowPrompt)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn UpdateBlocksActivation(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateBlocksActivation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AutomaticBackgroundTask(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticBackgroundTask)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAutoRepairEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAutoRepairEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Version(&self) -> ::windows_core::Result<PackageVersion> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageVersion>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVersion>(result__)
        }
    }
    pub fn LastChecked(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastChecked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn PausedUntil(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PausedUntil)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RepairUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RepairUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OptionalPackageUris)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Uri>>(result__)
        }
    }
    pub fn PolicySource(&self) -> ::windows_core::Result<AppInstallerPolicySource> {
        let this = &::windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppInstallerPolicySource>::zeroed();
            (::windows_core::Interface::vtable(this).PolicySource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallerPolicySource>(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallerInfo {}
impl ::core::fmt::Debug for AppInstallerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallerInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInstallerInfo;{29ab2ac0-d4f6-42a3-adcd-d6583c659508})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstallerInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstallerInfo";
}
impl ::core::convert::From<AppInstallerInfo> for ::windows_core::IUnknown {
    fn from(value: AppInstallerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerInfo> for ::windows_core::IUnknown {
    fn from(value: &AppInstallerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstallerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstallerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstallerInfo> for ::windows_core::IInspectable {
    fn from(value: AppInstallerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstallerInfo> for ::windows_core::IInspectable {
    fn from(value: &AppInstallerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstallerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstallerInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstallerInfo {}
unsafe impl ::core::marker::Sync for AppInstallerInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppInstallerPolicySource(pub i32);
impl AppInstallerPolicySource {
    pub const Default: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl ::core::marker::Copy for AppInstallerPolicySource {}
impl ::core::clone::Clone for AppInstallerPolicySource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallerPolicySource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppInstallerPolicySource {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallerPolicySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerPolicySource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstallerPolicySource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppInstallerPolicySource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppInstance(::windows_core::IUnknown);
impl AppInstance {
    pub fn Key(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Key)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsCurrentInstance(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentInstance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RedirectActivationTo(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RedirectActivationTo)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RecommendedInstance() -> ::windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecommendedInstance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstance>(result__)
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn GetActivatedEventArgs() -> ::windows_core::Result<Activation::IActivatedEventArgs> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetActivatedEventArgs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Activation::IActivatedEventArgs>(result__)
        })
    }
    pub fn FindOrRegisterInstanceForKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(key: Param0) -> ::windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindOrRegisterInstanceForKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppInstance>(result__)
        })
    }
    pub fn Unregister() -> ::windows_core::Result<()> {
        Self::IAppInstanceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInstances() -> ::windows_core::Result<::winrt_foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInstances)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppInstance, IAppInstanceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstance {}
impl ::core::fmt::Debug for AppInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.AppInstance;{675f2b47-f25f-4532-9fd6-3633e0634d01})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows_core::GUID = <IAppInstance as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppInstance {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstance";
}
impl ::core::convert::From<AppInstance> for ::windows_core::IUnknown {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows_core::IUnknown {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstance> for ::windows_core::IInspectable {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows_core::IInspectable {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
pub struct CameraApplicationManager;
impl CameraApplicationManager {
    pub fn ShowInstalledApplicationsUI() -> ::windows_core::Result<()> {
        Self::ICameraApplicationManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowInstalledApplicationsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ICameraApplicationManagerStatics<R, F: FnOnce(&ICameraApplicationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CameraApplicationManager, ICameraApplicationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CameraApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.CameraApplicationManager";
}
pub struct DesignMode;
impl DesignMode {
    pub fn DesignModeEnabled() -> ::windows_core::Result<bool> {
        Self::IDesignModeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DesignModeEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn DesignMode2Enabled() -> ::windows_core::Result<bool> {
        Self::IDesignModeStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DesignMode2Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IDesignModeStatics<R, F: FnOnce(&IDesignModeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DesignMode, IDesignModeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDesignModeStatics2<R, F: FnOnce(&IDesignModeStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DesignMode, IDesignModeStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for DesignMode {
    const NAME: &'static str = "Windows.ApplicationModel.DesignMode";
}
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(::windows_core::IUnknown);
impl EnteredBackgroundEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnteredBackgroundEventArgs {}
impl ::core::fmt::Debug for EnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.EnteredBackgroundEventArgs;{f722dcc2-9827-403d-aaed-ecca9ac17398})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IEnteredBackgroundEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.EnteredBackgroundEventArgs";
}
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<EnteredBackgroundEventArgs> for IEnteredBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: EnteredBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for IEnteredBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnteredBackgroundEventArgs> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IEnteredBackgroundEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IEnteredBackgroundEventArgs> for &EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IEnteredBackgroundEventArgs> {
        ::core::convert::TryInto::<IEnteredBackgroundEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FullTrustLaunchResult(pub i32);
impl FullTrustLaunchResult {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for FullTrustLaunchResult {}
impl ::core::clone::Clone for FullTrustLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FullTrustLaunchResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FullTrustLaunchResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for FullTrustLaunchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullTrustLaunchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FullTrustLaunchResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.FullTrustLaunchResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FullTrustProcessLaunchResult(::windows_core::IUnknown);
impl FullTrustProcessLaunchResult {
    pub fn LaunchResult(&self) -> ::windows_core::Result<FullTrustLaunchResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FullTrustLaunchResult>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FullTrustLaunchResult>(result__)
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
impl ::core::clone::Clone for FullTrustProcessLaunchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullTrustProcessLaunchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullTrustProcessLaunchResult {}
impl ::core::fmt::Debug for FullTrustProcessLaunchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullTrustProcessLaunchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FullTrustProcessLaunchResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.FullTrustProcessLaunchResult;{8917d888-edfb-515f-8e22-5ebceb69dfd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
    const IID: ::windows_core::GUID = <IFullTrustProcessLaunchResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FullTrustProcessLaunchResult {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLaunchResult";
}
impl ::core::convert::From<FullTrustProcessLaunchResult> for ::windows_core::IUnknown {
    fn from(value: FullTrustProcessLaunchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullTrustProcessLaunchResult> for ::windows_core::IUnknown {
    fn from(value: &FullTrustProcessLaunchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullTrustProcessLaunchResult> for ::windows_core::IInspectable {
    fn from(value: FullTrustProcessLaunchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullTrustProcessLaunchResult> for ::windows_core::IInspectable {
    fn from(value: &FullTrustProcessLaunchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FullTrustProcessLaunchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FullTrustProcessLaunchResult {}
unsafe impl ::core::marker::Sync for FullTrustProcessLaunchResult {}
pub struct FullTrustProcessLauncher;
impl FullTrustProcessLauncher {
    pub fn LaunchFullTrustProcessForCurrentAppAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn LaunchFullTrustProcessForCurrentAppWithParametersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(parametergroupid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithParametersAsync)(::windows_core::Interface::as_raw(this), parametergroupid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn LaunchFullTrustProcessForAppAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(fulltrustpackagerelativeappid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppAsync)(::windows_core::Interface::as_raw(this), fulltrustpackagerelativeappid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn LaunchFullTrustProcessForAppWithParametersAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(fulltrustpackagerelativeappid: Param0, parametergroupid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppWithParametersAsync)(::windows_core::Interface::as_raw(this), fulltrustpackagerelativeappid.into_param().abi(), parametergroupid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        })
    }
    pub fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(commandline: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithArgumentsAsync)(::windows_core::Interface::as_raw(this), commandline.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FullTrustProcessLaunchResult>>(result__)
        })
    }
    pub fn LaunchFullTrustProcessForAppWithArgumentsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(fulltrustpackagerelativeappid: Param0, commandline: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppWithArgumentsAsync)(::windows_core::Interface::as_raw(this), fulltrustpackagerelativeappid.into_param().abi(), commandline.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<FullTrustProcessLaunchResult>>(result__)
        })
    }
    pub fn IFullTrustProcessLauncherStatics<R, F: FnOnce(&IFullTrustProcessLauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFullTrustProcessLauncherStatics2<R, F: FnOnce(&IFullTrustProcessLauncherStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for FullTrustProcessLauncher {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLauncher";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDisplayInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1aeb1103_e4d4_41aa_a4f6_c4a276e79eac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDisplayInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetLogo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInfo {
    type Vtable = IAppInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf7f59b3_6a09_4de8_a6c0_5792d56880d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInfo2 {
    type Vtable = IAppInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe4b1f5a_2098_431b_bd25_b30878748d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInfo3 {
    type Vtable = IAppInfo3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09a78e46_93a4_46de_9397_0843b57115ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExecutionContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppExecutionContext) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfo4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInfo4 {
    type Vtable = IAppInfo4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f34bdeb_1609_4554_9f33_12e1e803e0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfo4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SupportedFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInfoStatics {
    type Vtable = IAppInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf1f782a_e48b_4f0c_9b0b_79c3f8957dd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFromAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFromAppUserModelIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFromAppUserModelIdForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29ab2ac0_d4f6_42a3_adcd_d6583c659508);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallerInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallerInfo2 {
    type Vtable = IAppInstallerInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd20f1388_8256_597c_8511_c84ec50d5e2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows_core::HRESULT,
    pub LastChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub PausedUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RepairUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RepairUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    pub PolicySource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallerPolicySource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstance(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x675f2b47_f25f_4532_9fd6_3633e0634d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d11e77f_9ea6_47af_a6ec_46784c5ba254);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecommendedInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub GetActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    GetActivatedEventArgs: usize,
    pub FindOrRegisterInstanceForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInstances: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraApplicationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICameraApplicationManagerStatics {
    type Vtable = ICameraApplicationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9599ddce_9bd3_435c_8054_c1add50028fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraApplicationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignModeStatics {
    type Vtable = IDesignModeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c3893cc_f81a_4e7a_b857_76a80887e185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesignModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesignModeStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesignModeStatics2 {
    type Vtable = IDesignModeStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80cf8137_b064_4858_bec8_3eba22357535);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesignModeStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesignMode2Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IEnteredBackgroundEventArgs(::windows_core::IUnknown);
impl IEnteredBackgroundEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::convert::From<IEnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: IEnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IEnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: IEnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IEnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IEnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnteredBackgroundEventArgs {}
impl ::core::fmt::Debug for IEnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IEnteredBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f722dcc2-9827-403d-aaed-ecca9ac17398}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IEnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf722dcc2_9827_403d_aaed_ecca9ac17398);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnteredBackgroundEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLaunchResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8917d888_edfb_515f_8e22_5ebceb69dfd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLaunchResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LaunchResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FullTrustLaunchResult) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullTrustProcessLauncherStatics {
    type Vtable = IFullTrustProcessLauncherStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd784837f_1100_3c6b_a455_f6262cc331b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LaunchFullTrustProcessForCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFullTrustProcessForCurrentAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parametergroupid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, parametergroupid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullTrustProcessLauncherStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullTrustProcessLauncherStatics2 {
    type Vtable = IFullTrustProcessLauncherStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b8ed72f_b65c_56cf_a1a7_2bf77cbc6ea8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullTrustProcessLauncherStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandline: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppWithArgumentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, commandline: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILeavingBackgroundEventArgs(::windows_core::IUnknown);
impl ILeavingBackgroundEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::convert::From<ILeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: ILeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ILeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: ILeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ILeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILeavingBackgroundEventArgs {}
impl ::core::fmt::Debug for ILeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILeavingBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39c6ec9a_ae6e_46f9_a07a_cfc23f88733e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILeavingBackgroundEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeatureRequestResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd45156a6_1e24_5ddd_abb4_6188aba4d5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeatureRequestResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FeatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LimitedAccessFeatureStatus) -> ::windows_core::HRESULT,
    pub EstimatedRemovalDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimitedAccessFeaturesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILimitedAccessFeaturesStatics {
    type Vtable = ILimitedAccessFeaturesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8be612d4_302b_5fbf_a632_1a99e43e8925);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimitedAccessFeaturesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryUnlockFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featureid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, attestation: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage {
    type Vtable = IPackage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x163c792f_bd75_413c_bf23_b1fe7b95d825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub InstalledLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstalledLocation: usize,
    pub IsFramework: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Dependencies: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage2 {
    type Vtable = IPackage2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6612fb6_7688_4ace_95fb_359538e7aa01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PublisherDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsResourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDevelopmentMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage3 {
    type Vtable = IPackage3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f738b61_f86a_4917_93d1_f1ee9d3b35d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InstalledDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntriesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage4 {
    type Vtable = IPackage4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65aed1ae_b95b_450c_882b_6255187f397e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SignatureKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageSignatureKind) -> ::windows_core::HRESULT,
    pub IsOptional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub VerifyContentIntegrityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage5 {
    type Vtable = IPackage5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e842dd4_d9ac_45ed_9a1e_74ce056b2635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContentGroupsAsync: usize,
    pub GetContentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsWithPriorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: ::windows_core::RawPtr, movetoheadofqueue: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsWithPriorityAsync: usize,
    pub SetInUseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inuse: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage6 {
    type Vtable = IPackage6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b1ad942_12d7_4754_ae4e_638cbc0e3a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAppInstallerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckUpdateAvailabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage7 {
    type Vtable = IPackage7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86ff8d31_a2e4_45e0_9732_283a6d88fde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub MutableLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MutableLocation: usize,
    #[cfg(feature = "Storage")]
    pub EffectiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveLocation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackage8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackage8 {
    type Vtable = IPackage8_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c584f7b_ce2a_4be6_a093_77cfbb2a7ea1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackage8_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub EffectiveExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub MachineExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MachineExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub UserExternalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    UserExternalLocation: usize,
    pub InstalledPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EffectivePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EffectiveExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MachineExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserExternalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetLogoAsRandomAccessStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: ::winrt_foundation::Size, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetLogoAsRandomAccessStreamReference: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntries: usize,
    pub IsStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x230a3751_9de3_4445_be74_91fb325abefe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageInstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageUninstalling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalog2 {
    type Vtable = IPackageCatalog2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96a60c36_8ff7_4344_b6bf_ee64c2207ed2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePackageContentGroupStaging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AddOptionalPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalog3 {
    type Vtable = IPackageCatalog3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96dd5c88_8837_43f9_9015_033434ba14f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackagefamilynames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalog4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalog4 {
    type Vtable = IPackageCatalog4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc37c399b_44cc_4b7b_8baf_796c04ead3b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalog4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddResourcePackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackagefamilyname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, resourceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: AddResourcePackageOptions, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveResourcePackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcepackages: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveResourcePackagesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddOptionalPackageResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bf10cd4_b4df_47b3_a963_e2fa832f7dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddOptionalPackageResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogAddResourcePackageResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9636ce0d_3e17_493f_aa08_ccec6fdef699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogAddResourcePackageResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveOptionalPackagesResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29d2f97b_d974_4e64_9359_22cadfd79828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveOptionalPackagesResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogRemoveResourcePackagesResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae719709_1a52_4321_87b3_e5a1a17981a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogRemoveResourcePackagesResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageCatalogStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageCatalogStatics {
    type Vtable = IPackageCatalogStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa18c9696_e65b_4634_ba21_5e63eb7244a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageCatalogStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenForCurrentPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f62695d_120a_4798_b5e1_5800dda8f2e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageContentGroupState) -> ::windows_core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStagingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d7bc27e_6f27_446c_986e_d4733d4d9113);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStagingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub ContentGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsContentGroupRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageContentGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageContentGroupStatics {
    type Vtable = IPackageContentGroupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70ee7619_5f12_4b92_b9ea_6ccada13bc75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageContentGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequiredGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageId {
    type Vtable = IPackageId_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1adb665e_37c7_4790_9980_dd7ae74e8bb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageId_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub Architecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_system::ProcessorArchitecture) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    Architecture: usize,
    pub ResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PublisherId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageIdWithMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageIdWithMetadata {
    type Vtable = IPackageIdWithMetadata_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40577a7c_0c9e_443d_9074_855f5ce0a08d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageIdWithMetadata_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageInstallingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97741eb7_ab7a_401a_8b61_eb0e7faff237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageInstallingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStagingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1041682d_54e2_4f51_b828_9ef7046c210f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStagingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageStatics {
    type Vtable = IPackageStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e534bdf_2960_4878_97a4_9624deb72f2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageStatus {
    type Vtable = IPackageStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fe74f71_a365_4c09_a02d_046d525ea1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VerifyIsOK: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NotAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PackageOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DataOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub NeedsRemediation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub LicenseIssue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Tampered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DependencyIssue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Servicing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DeploymentInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatus2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageStatus2 {
    type Vtable = IPackageStatus2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf428fa93_7c56_4862_acfa_abaedcc0694d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatus2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPartiallyStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x437d714d_bd80_4a70_bc50_f6e796509575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUninstallingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4443aa52_ab22_44cd_82bb_4ec9b827367a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUninstallingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdateAvailabilityResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x114e5009_199a_48a1_a079_313c45634a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdateAvailabilityResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PackageUpdateAvailability) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageUpdatingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd7b4228_fd74_443e_b114_23e677b0e86f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUpdatingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivityId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TargetPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageWithMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageWithMetadata {
    type Vtable = IPackageWithMetadata_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95949780_1de9_40f2_b452_0de9f1910012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageWithMetadata_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InstallDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub GetThumbnailToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Launch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTask(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStartupTask {
    type Vtable = IStartupTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf75c23c8_b5f2_4f6c_88dd_36cb1d599d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTask_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StartupTaskState) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartupTaskStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStartupTaskStatics {
    type Vtable = IStartupTaskStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee5b60bd_a148_41a7_b26e_e8b88a1e62f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForCurrentPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForCurrentPackageAsync: usize,
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISuspendingDeferral(::windows_core::IUnknown);
impl ISuspendingDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<ISuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: ISuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: &ISuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: ISuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: &ISuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingDeferral {}
impl ::core::fmt::Debug for ISuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISuspendingDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{59140509-8bc9-4eb4-b636-dabdc4f46f66}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59140509_8bc9_4eb4_b636_dabdc4f46f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISuspendingEventArgs(::windows_core::IUnknown);
impl ISuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> ::windows_core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuspendingOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SuspendingOperation>(result__)
        }
    }
}
impl ::core::convert::From<ISuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: ISuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ISuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: ISuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ISuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingEventArgs {}
impl ::core::fmt::Debug for ISuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISuspendingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{96061c05-2dba-4d08-b0bd-2b30a131c6aa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96061c05_2dba_4d08_b0bd_2b30a131c6aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SuspendingOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISuspendingOperation(::windows_core::IUnknown);
impl ISuspendingOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SuspendingDeferral>(result__)
        }
    }
    pub fn Deadline(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::convert::From<ISuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: ISuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: &ISuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: ISuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: &ISuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISuspendingOperation {}
impl ::core::fmt::Debug for ISuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISuspendingOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISuspendingOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9da4ca41-20e1-4e9b-9f65-a9f435340c3a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9da4ca41_20e1_4e9b_9f65_a9f435340c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuspendingOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(::windows_core::IUnknown);
impl LeavingBackgroundEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LeavingBackgroundEventArgs {}
impl ::core::fmt::Debug for LeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LeavingBackgroundEventArgs;{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILeavingBackgroundEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LeavingBackgroundEventArgs";
}
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LeavingBackgroundEventArgs> for ILeavingBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LeavingBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for ILeavingBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILeavingBackgroundEventArgs> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILeavingBackgroundEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILeavingBackgroundEventArgs> for &LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILeavingBackgroundEventArgs> {
        ::core::convert::TryInto::<ILeavingBackgroundEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[repr(transparent)]
pub struct LimitedAccessFeatureRequestResult(::windows_core::IUnknown);
impl LimitedAccessFeatureRequestResult {
    pub fn FeatureId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FeatureId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<LimitedAccessFeatureStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LimitedAccessFeatureStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LimitedAccessFeatureStatus>(result__)
        }
    }
    pub fn EstimatedRemovalDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedRemovalDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for LimitedAccessFeatureRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LimitedAccessFeatureRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LimitedAccessFeatureRequestResult {}
impl ::core::fmt::Debug for LimitedAccessFeatureRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimitedAccessFeatureRequestResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LimitedAccessFeatureRequestResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.LimitedAccessFeatureRequestResult;{d45156a6-1e24-5ddd-abb4-6188aba4d5bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
    const IID: ::windows_core::GUID = <ILimitedAccessFeatureRequestResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LimitedAccessFeatureRequestResult {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
}
impl ::core::convert::From<LimitedAccessFeatureRequestResult> for ::windows_core::IUnknown {
    fn from(value: LimitedAccessFeatureRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimitedAccessFeatureRequestResult> for ::windows_core::IUnknown {
    fn from(value: &LimitedAccessFeatureRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LimitedAccessFeatureRequestResult> for ::windows_core::IInspectable {
    fn from(value: LimitedAccessFeatureRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimitedAccessFeatureRequestResult> for ::windows_core::IInspectable {
    fn from(value: &LimitedAccessFeatureRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LimitedAccessFeatureRequestResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LimitedAccessFeatureRequestResult {}
unsafe impl ::core::marker::Sync for LimitedAccessFeatureRequestResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LimitedAccessFeatureStatus(pub i32);
impl LimitedAccessFeatureStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
    pub const AvailableWithoutToken: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LimitedAccessFeatureStatus {}
impl ::core::clone::Clone for LimitedAccessFeatureStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LimitedAccessFeatureStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LimitedAccessFeatureStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LimitedAccessFeatureStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimitedAccessFeatureStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LimitedAccessFeatureStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.LimitedAccessFeatureStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct LimitedAccessFeatures;
impl LimitedAccessFeatures {
    pub fn TryUnlockFeature<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(featureid: Param0, token: Param1, attestation: Param2) -> ::windows_core::Result<LimitedAccessFeatureRequestResult> {
        Self::ILimitedAccessFeaturesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUnlockFeature)(::windows_core::Interface::as_raw(this), featureid.into_param().abi(), token.into_param().abi(), attestation.into_param().abi(), result__.as_mut_ptr()).from_abi::<LimitedAccessFeatureRequestResult>(result__)
        })
    }
    pub fn ILimitedAccessFeaturesStatics<R, F: FnOnce(&ILimitedAccessFeaturesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LimitedAccessFeatures, ILimitedAccessFeaturesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for LimitedAccessFeatures {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatures";
}
#[repr(transparent)]
pub struct Package(::windows_core::IUnknown);
impl Package {
    pub fn Id(&self) -> ::windows_core::Result<PackageId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageId>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn InstalledLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InstalledLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    pub fn IsFramework(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsFramework)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Dependencies(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dependencies)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Package>>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PublisherDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PublisherDisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Logo(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn IsResourcePackage(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsResourcePackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBundle(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBundle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDevelopmentMode(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDevelopmentMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<PackageStatus> {
        let this = &::windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageStatus>(result__)
        }
    }
    pub fn InstalledDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).InstalledDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntriesAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Core::AppListEntry>>> {
        let this = &::windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppListEntriesAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Core::AppListEntry>>>(result__)
        }
    }
    pub fn SignatureKind(&self) -> ::windows_core::Result<PackageSignatureKind> {
        let this = &::windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageSignatureKind>::zeroed();
            (::windows_core::Interface::vtable(this).SignatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageSignatureKind>(result__)
        }
    }
    pub fn IsOptional(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOptional)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn VerifyContentIntegrityAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerifyContentIntegrityAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContentGroupsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContentGroupsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    pub fn GetContentGroupAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageContentGroup>> {
        let this = &::windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetContentGroupAsync)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageContentGroup>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, names: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StageContentGroupsAsync)(::windows_core::Interface::as_raw(this), names.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsWithPriorityAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, names: Param0, movetoheadofqueue: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &::windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StageContentGroupsWithPriorityAsync)(::windows_core::Interface::as_raw(this), names.into_param().abi(), movetoheadofqueue, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVector<PackageContentGroup>>>(result__)
        }
    }
    pub fn SetInUseAsync(&self, inuse: bool) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetInUseAsync)(::windows_core::Interface::as_raw(this), inuse, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetAppInstallerInfo(&self) -> ::windows_core::Result<AppInstallerInfo> {
        let this = &::windows_core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppInstallerInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppInstallerInfo>(result__)
        }
    }
    pub fn CheckUpdateAvailabilityAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageUpdateAvailabilityResult>> {
        let this = &::windows_core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckUpdateAvailabilityAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageUpdateAvailabilityResult>>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn MutableLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MutableLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn EffectiveLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectiveLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn EffectiveExternalLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectiveExternalLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn MachineExternalLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MachineExternalLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn UserExternalLocation(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserExternalLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
    pub fn InstalledPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).InstalledPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MutablePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MutablePath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EffectivePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EffectivePath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EffectiveExternalPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EffectiveExternalPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MachineExternalPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MachineExternalPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn UserExternalPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).UserExternalPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetLogoAsRandomAccessStreamReference<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, size: Param0) -> ::windows_core::Result<::winrt_storage::Streams::RandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLogoAsRandomAccessStreamReference)(::windows_core::Interface::as_raw(this), size.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntries(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Core::AppListEntry>> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppListEntries)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Core::AppListEntry>>(result__)
        }
    }
    pub fn IsStub(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStub)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<Package> {
        Self::IPackageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        })
    }
    pub fn InstallDate(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = &::windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).InstallDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn GetThumbnailToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailToken)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Launch<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, parameters: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Launch)(::windows_core::Interface::as_raw(this), parameters.into_param().abi()).ok() }
    }
    pub fn IPackageStatics<R, F: FnOnce(&IPackageStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Package, IPackageStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Package {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Package {}
impl ::core::fmt::Debug for Package {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Package").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Package {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Package;{163c792f-bd75-413c-bf23-b1fe7b95d825})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Package {
    type Vtable = IPackage_Vtbl;
    const IID: ::windows_core::GUID = <IPackage as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Package {
    const NAME: &'static str = "Windows.ApplicationModel.Package";
}
impl ::core::convert::From<Package> for ::windows_core::IUnknown {
    fn from(value: Package) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Package> for ::windows_core::IUnknown {
    fn from(value: &Package) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Package {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Package {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Package> for ::windows_core::IInspectable {
    fn from(value: Package) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Package> for ::windows_core::IInspectable {
    fn from(value: &Package) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Package {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Package {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Package {}
unsafe impl ::core::marker::Sync for Package {}
#[repr(transparent)]
pub struct PackageCatalog(::windows_core::IUnknown);
impl PackageCatalog {
    pub fn PackageStaging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageStaging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageStaging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageStaging)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PackageInstalling<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageInstalling)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageInstalling<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageInstalling)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PackageUpdating<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageUpdating)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageUpdating<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUpdating)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PackageUninstalling<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageUninstalling)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageUninstalling<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageUninstalling)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PackageStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PackageContentGroupStaging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PackageContentGroupStaging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePackageContentGroupStaging<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePackageContentGroupStaging)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AddOptionalPackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, optionalpackagefamilyname: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddOptionalPackageAsync)(::windows_core::Interface::as_raw(this), optionalpackagefamilyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveOptionalPackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, optionalpackagefamilynames: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveOptionalPackagesAsync)(::windows_core::Interface::as_raw(this), optionalpackagefamilynames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>(result__)
        }
    }
    pub fn AddResourcePackageAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, resourcepackagefamilyname: Param0, resourceid: Param1, options: AddResourcePackageOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddResourcePackageAsync)(::windows_core::Interface::as_raw(this), resourcepackagefamilyname.into_param().abi(), resourceid.into_param().abi(), options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveResourcePackagesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Package>>>(&self, resourcepackages: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>> {
        let this = &::windows_core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveResourcePackagesAsync)(::windows_core::Interface::as_raw(this), resourcepackages.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>(result__)
        }
    }
    pub fn OpenForCurrentPackage() -> ::windows_core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenForCurrentPackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageCatalog>(result__)
        })
    }
    pub fn OpenForCurrentUser() -> ::windows_core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenForCurrentUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageCatalog>(result__)
        })
    }
    pub fn IPackageCatalogStatics<R, F: FnOnce(&IPackageCatalogStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PackageCatalog, IPackageCatalogStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PackageCatalog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalog {}
impl ::core::fmt::Debug for PackageCatalog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalog").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageCatalog {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalog;{230a3751-9de3-4445-be74-91fb325abefe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
    const IID: ::windows_core::GUID = <IPackageCatalog as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalog";
}
impl ::core::convert::From<PackageCatalog> for ::windows_core::IUnknown {
    fn from(value: PackageCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalog> for ::windows_core::IUnknown {
    fn from(value: &PackageCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalog> for ::windows_core::IInspectable {
    fn from(value: PackageCatalog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalog> for ::windows_core::IInspectable {
    fn from(value: &PackageCatalog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageCatalog {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PackageCatalogAddOptionalPackageResult(::windows_core::IUnknown);
impl PackageCatalogAddOptionalPackageResult {
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
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
impl ::core::clone::Clone for PackageCatalogAddOptionalPackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogAddOptionalPackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogAddOptionalPackageResult {}
impl ::core::fmt::Debug for PackageCatalogAddOptionalPackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogAddOptionalPackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageCatalogAddOptionalPackageResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult;{3bf10cd4-b4df-47b3-a963-e2fa832f7dd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageCatalogAddOptionalPackageResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageCatalogAddOptionalPackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
}
impl ::core::convert::From<PackageCatalogAddOptionalPackageResult> for ::windows_core::IUnknown {
    fn from(value: PackageCatalogAddOptionalPackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddOptionalPackageResult> for ::windows_core::IUnknown {
    fn from(value: &PackageCatalogAddOptionalPackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogAddOptionalPackageResult> for ::windows_core::IInspectable {
    fn from(value: PackageCatalogAddOptionalPackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddOptionalPackageResult> for ::windows_core::IInspectable {
    fn from(value: &PackageCatalogAddOptionalPackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageCatalogAddOptionalPackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PackageCatalogAddResourcePackageResult(::windows_core::IUnknown);
impl PackageCatalogAddResourcePackageResult {
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
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
impl ::core::clone::Clone for PackageCatalogAddResourcePackageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogAddResourcePackageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogAddResourcePackageResult {}
impl ::core::fmt::Debug for PackageCatalogAddResourcePackageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogAddResourcePackageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageCatalogAddResourcePackageResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogAddResourcePackageResult;{9636ce0d-3e17-493f-aa08-ccec6fdef699})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageCatalogAddResourcePackageResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageCatalogAddResourcePackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
}
impl ::core::convert::From<PackageCatalogAddResourcePackageResult> for ::windows_core::IUnknown {
    fn from(value: PackageCatalogAddResourcePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddResourcePackageResult> for ::windows_core::IUnknown {
    fn from(value: &PackageCatalogAddResourcePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogAddResourcePackageResult> for ::windows_core::IInspectable {
    fn from(value: PackageCatalogAddResourcePackageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogAddResourcePackageResult> for ::windows_core::IInspectable {
    fn from(value: &PackageCatalogAddResourcePackageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageCatalogAddResourcePackageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageCatalogAddResourcePackageResult {}
unsafe impl ::core::marker::Sync for PackageCatalogAddResourcePackageResult {}
#[repr(transparent)]
pub struct PackageCatalogRemoveOptionalPackagesResult(::windows_core::IUnknown);
impl PackageCatalogRemoveOptionalPackagesResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PackagesRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Package>>(result__)
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
impl ::core::clone::Clone for PackageCatalogRemoveOptionalPackagesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogRemoveOptionalPackagesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogRemoveOptionalPackagesResult {}
impl ::core::fmt::Debug for PackageCatalogRemoveOptionalPackagesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogRemoveOptionalPackagesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageCatalogRemoveOptionalPackagesResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult;{29d2f97b-d974-4e64-9359-22cadfd79828})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageCatalogRemoveOptionalPackagesResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageCatalogRemoveOptionalPackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
}
impl ::core::convert::From<PackageCatalogRemoveOptionalPackagesResult> for ::windows_core::IUnknown {
    fn from(value: PackageCatalogRemoveOptionalPackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveOptionalPackagesResult> for ::windows_core::IUnknown {
    fn from(value: &PackageCatalogRemoveOptionalPackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogRemoveOptionalPackagesResult> for ::windows_core::IInspectable {
    fn from(value: PackageCatalogRemoveOptionalPackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveOptionalPackagesResult> for ::windows_core::IInspectable {
    fn from(value: &PackageCatalogRemoveOptionalPackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageCatalogRemoveOptionalPackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct PackageCatalogRemoveResourcePackagesResult(::windows_core::IUnknown);
impl PackageCatalogRemoveResourcePackagesResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PackagesRemoved)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Package>>(result__)
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
impl ::core::clone::Clone for PackageCatalogRemoveResourcePackagesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageCatalogRemoveResourcePackagesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageCatalogRemoveResourcePackagesResult {}
impl ::core::fmt::Debug for PackageCatalogRemoveResourcePackagesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageCatalogRemoveResourcePackagesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageCatalogRemoveResourcePackagesResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult;{ae719709-1a52-4321-87b3-e5a1a17981a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageCatalogRemoveResourcePackagesResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageCatalogRemoveResourcePackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
}
impl ::core::convert::From<PackageCatalogRemoveResourcePackagesResult> for ::windows_core::IUnknown {
    fn from(value: PackageCatalogRemoveResourcePackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveResourcePackagesResult> for ::windows_core::IUnknown {
    fn from(value: &PackageCatalogRemoveResourcePackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageCatalogRemoveResourcePackagesResult> for ::windows_core::IInspectable {
    fn from(value: PackageCatalogRemoveResourcePackagesResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageCatalogRemoveResourcePackagesResult> for ::windows_core::IInspectable {
    fn from(value: &PackageCatalogRemoveResourcePackagesResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageCatalogRemoveResourcePackagesResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageCatalogRemoveResourcePackagesResult {}
unsafe impl ::core::marker::Sync for PackageCatalogRemoveResourcePackagesResult {}
#[repr(transparent)]
pub struct PackageContentGroup(::windows_core::IUnknown);
impl PackageContentGroup {
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<PackageContentGroupState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageContentGroupState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageContentGroupState>(result__)
        }
    }
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RequiredGroupName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPackageContentGroupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RequiredGroupName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IPackageContentGroupStatics<R, F: FnOnce(&IPackageContentGroupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PackageContentGroup, IPackageContentGroupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PackageContentGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageContentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageContentGroup {}
impl ::core::fmt::Debug for PackageContentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageContentGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageContentGroup;{8f62695d-120a-4798-b5e1-5800dda8f2e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
    const IID: ::windows_core::GUID = <IPackageContentGroup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageContentGroup {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroup";
}
impl ::core::convert::From<PackageContentGroup> for ::windows_core::IUnknown {
    fn from(value: PackageContentGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroup> for ::windows_core::IUnknown {
    fn from(value: &PackageContentGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageContentGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageContentGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageContentGroup> for ::windows_core::IInspectable {
    fn from(value: PackageContentGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroup> for ::windows_core::IInspectable {
    fn from(value: &PackageContentGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageContentGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageContentGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageContentGroup {}
unsafe impl ::core::marker::Sync for PackageContentGroup {}
#[repr(transparent)]
pub struct PackageContentGroupStagingEventArgs(::windows_core::IUnknown);
impl PackageContentGroupStagingEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn ContentGroupName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentGroupName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsContentGroupRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentGroupRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageContentGroupStagingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageContentGroupStagingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageContentGroupStagingEventArgs {}
impl ::core::fmt::Debug for PackageContentGroupStagingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroupStagingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageContentGroupStagingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageContentGroupStagingEventArgs;{3d7bc27e-6f27-446c-986e-d4733d4d9113})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageContentGroupStagingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageContentGroupStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
}
impl ::core::convert::From<PackageContentGroupStagingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageContentGroupStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroupStagingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageContentGroupStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageContentGroupStagingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageContentGroupStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageContentGroupStagingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageContentGroupStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageContentGroupStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageContentGroupStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageContentGroupStagingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageContentGroupState(pub i32);
impl PackageContentGroupState {
    pub const NotStaged: Self = Self(0i32);
    pub const Queued: Self = Self(1i32);
    pub const Staging: Self = Self(2i32);
    pub const Staged: Self = Self(3i32);
}
impl ::core::marker::Copy for PackageContentGroupState {}
impl ::core::clone::Clone for PackageContentGroupState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageContentGroupState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageContentGroupState {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageContentGroupState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageContentGroupState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageContentGroupState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageContentGroupState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PackageId(::windows_core::IUnknown);
impl PackageId {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Version(&self) -> ::windows_core::Result<PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageVersion>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageVersion>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn Architecture(&self) -> ::windows_core::Result<::winrt_system::ProcessorArchitecture> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_system::ProcessorArchitecture>::zeroed();
            (::windows_core::Interface::vtable(this).Architecture)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::ProcessorArchitecture>(result__)
        }
    }
    pub fn ResourceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ResourceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Publisher(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Publisher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn PublisherId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PublisherId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FullName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn FamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Author(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageId {}
impl ::core::fmt::Debug for PackageId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageId").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageId;{1adb665e-37c7-4790-9980-dd7ae74e8bb2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageId {
    type Vtable = IPackageId_Vtbl;
    const IID: ::windows_core::GUID = <IPackageId as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageId {
    const NAME: &'static str = "Windows.ApplicationModel.PackageId";
}
impl ::core::convert::From<PackageId> for ::windows_core::IUnknown {
    fn from(value: PackageId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageId> for ::windows_core::IUnknown {
    fn from(value: &PackageId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageId> for ::windows_core::IInspectable {
    fn from(value: PackageId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageId> for ::windows_core::IInspectable {
    fn from(value: &PackageId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageId {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageId {}
unsafe impl ::core::marker::Sync for PackageId {}
#[repr(C)]
pub struct PackageInstallProgress {
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for PackageInstallProgress {}
impl ::core::clone::Clone for PackageInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PackageInstallProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PackageInstallProgress").field("PercentComplete", &self.PercentComplete).finish()
    }
}
unsafe impl ::windows_core::Abi for PackageInstallProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for PackageInstallProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageInstallProgress;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PackageInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PackageInstallProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for PackageInstallProgress {}
impl ::core::default::Default for PackageInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct PackageInstallingEventArgs(::windows_core::IUnknown);
impl PackageInstallingEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageInstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageInstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageInstallingEventArgs {}
impl ::core::fmt::Debug for PackageInstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageInstallingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageInstallingEventArgs;{97741eb7-ab7a-401a-8b61-eb0e7faff237})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageInstallingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageInstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageInstallingEventArgs";
}
impl ::core::convert::From<PackageInstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageInstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageInstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageInstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageInstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageInstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageInstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageInstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageInstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageInstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageInstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageInstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageInstallingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageSignatureKind(pub i32);
impl PackageSignatureKind {
    pub const None: Self = Self(0i32);
    pub const Developer: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
    pub const Store: Self = Self(3i32);
    pub const System: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageSignatureKind {}
impl ::core::clone::Clone for PackageSignatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageSignatureKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageSignatureKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageSignatureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageSignatureKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageSignatureKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageSignatureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PackageStagingEventArgs(::windows_core::IUnknown);
impl PackageStagingEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStagingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStagingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStagingEventArgs {}
impl ::core::fmt::Debug for PackageStagingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStagingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageStagingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStagingEventArgs;{1041682d-54e2-4f51-b828-9ef7046c210f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageStagingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStagingEventArgs";
}
impl ::core::convert::From<PackageStagingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStagingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStagingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageStagingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStagingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageStagingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageStagingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStagingEventArgs {}
unsafe impl ::core::marker::Sync for PackageStagingEventArgs {}
#[repr(transparent)]
pub struct PackageStatus(::windows_core::IUnknown);
impl PackageStatus {
    pub fn VerifyIsOK(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).VerifyIsOK)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NotAvailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).NotAvailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PackageOffline(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PackageOffline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DataOffline(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DataOffline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Disabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Disabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NeedsRemediation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).NeedsRemediation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn LicenseIssue(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).LicenseIssue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Modified(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Modified)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Tampered(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Tampered)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DependencyIssue(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DependencyIssue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Servicing(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Servicing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DeploymentInProgress(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DeploymentInProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPartiallyStaged(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPackageStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPartiallyStaged)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStatus {}
impl ::core::fmt::Debug for PackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStatus;{5fe74f71-a365-4c09-a02d-046d525ea1da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageStatus {
    type Vtable = IPackageStatus_Vtbl;
    const IID: ::windows_core::GUID = <IPackageStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageStatus {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatus";
}
impl ::core::convert::From<PackageStatus> for ::windows_core::IUnknown {
    fn from(value: PackageStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatus> for ::windows_core::IUnknown {
    fn from(value: &PackageStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStatus> for ::windows_core::IInspectable {
    fn from(value: PackageStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatus> for ::windows_core::IInspectable {
    fn from(value: &PackageStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStatus {}
unsafe impl ::core::marker::Sync for PackageStatus {}
#[repr(transparent)]
pub struct PackageStatusChangedEventArgs(::windows_core::IUnknown);
impl PackageStatusChangedEventArgs {
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageStatusChangedEventArgs {}
impl ::core::fmt::Debug for PackageStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageStatusChangedEventArgs;{437d714d-bd80-4a70-bc50-f6e796509575})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatusChangedEventArgs";
}
impl ::core::convert::From<PackageStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for PackageStatusChangedEventArgs {}
#[repr(transparent)]
pub struct PackageUninstallingEventArgs(::windows_core::IUnknown);
impl PackageUninstallingEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Package(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Package)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUninstallingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUninstallingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUninstallingEventArgs {}
impl ::core::fmt::Debug for PackageUninstallingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUninstallingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageUninstallingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUninstallingEventArgs;{4443aa52-ab22-44cd-82bb-4ec9b827367a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageUninstallingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUninstallingEventArgs";
}
impl ::core::convert::From<PackageUninstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUninstallingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUninstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageUninstallingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUninstallingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageUninstallingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageUninstallingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUninstallingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUninstallingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PackageUpdateAvailability(pub i32);
impl PackageUpdateAvailability {
    pub const Unknown: Self = Self(0i32);
    pub const NoUpdates: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Required: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for PackageUpdateAvailability {}
impl ::core::clone::Clone for PackageUpdateAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageUpdateAvailability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PackageUpdateAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageUpdateAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdateAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageUpdateAvailability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageUpdateAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PackageUpdateAvailabilityResult(::windows_core::IUnknown);
impl PackageUpdateAvailabilityResult {
    pub fn Availability(&self) -> ::windows_core::Result<PackageUpdateAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PackageUpdateAvailability>::zeroed();
            (::windows_core::Interface::vtable(this).Availability)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PackageUpdateAvailability>(result__)
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
impl ::core::clone::Clone for PackageUpdateAvailabilityResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUpdateAvailabilityResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUpdateAvailabilityResult {}
impl ::core::fmt::Debug for PackageUpdateAvailabilityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdateAvailabilityResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageUpdateAvailabilityResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUpdateAvailabilityResult;{114e5009-199a-48a1-a079-313c45634a71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
    const IID: ::windows_core::GUID = <IPackageUpdateAvailabilityResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageUpdateAvailabilityResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdateAvailabilityResult";
}
impl ::core::convert::From<PackageUpdateAvailabilityResult> for ::windows_core::IUnknown {
    fn from(value: PackageUpdateAvailabilityResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdateAvailabilityResult> for ::windows_core::IUnknown {
    fn from(value: &PackageUpdateAvailabilityResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUpdateAvailabilityResult> for ::windows_core::IInspectable {
    fn from(value: PackageUpdateAvailabilityResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdateAvailabilityResult> for ::windows_core::IInspectable {
    fn from(value: &PackageUpdateAvailabilityResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageUpdateAvailabilityResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUpdateAvailabilityResult {}
unsafe impl ::core::marker::Sync for PackageUpdateAvailabilityResult {}
#[repr(transparent)]
pub struct PackageUpdatingEventArgs(::windows_core::IUnknown);
impl PackageUpdatingEventArgs {
    pub fn ActivityId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).ActivityId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SourcePackage(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourcePackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn TargetPackage(&self) -> ::windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetPackage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Package>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for PackageUpdatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageUpdatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUpdatingEventArgs {}
impl ::core::fmt::Debug for PackageUpdatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUpdatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PackageUpdatingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.PackageUpdatingEventArgs;{cd7b4228-fd74-443e-b114-23e677b0e86f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPackageUpdatingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdatingEventArgs";
}
impl ::core::convert::From<PackageUpdatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: PackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PackageUpdatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: PackageUpdatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PackageUpdatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PackageUpdatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PackageUpdatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PackageUpdatingEventArgs {}
unsafe impl ::core::marker::Sync for PackageUpdatingEventArgs {}
#[repr(C)]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for PackageVersion {}
impl ::core::clone::Clone for PackageVersion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PackageVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PackageVersion").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows_core::Abi for PackageVersion {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for PackageVersion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageVersion;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PackageVersion {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PackageVersion>()) == 0 }
    }
}
impl ::core::cmp::Eq for PackageVersion {}
impl ::core::default::Default for PackageVersion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct StartupTask(::windows_core::IUnknown);
impl StartupTask {
    pub fn RequestEnableAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StartupTaskState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestEnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StartupTaskState>>(result__)
        }
    }
    pub fn Disable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Disable)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<StartupTaskState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<StartupTaskState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StartupTaskState>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForCurrentPackageAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StartupTask>>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentPackageAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<StartupTask>>>(result__)
        })
    }
    pub fn GetAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(taskid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<StartupTask>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAsync)(::windows_core::Interface::as_raw(this), taskid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<StartupTask>>(result__)
        })
    }
    pub fn IStartupTaskStatics<R, F: FnOnce(&IStartupTaskStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StartupTask, IStartupTaskStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StartupTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartupTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTask {}
impl ::core::fmt::Debug for StartupTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTask").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StartupTask {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.StartupTask;{f75c23c8-b5f2-4f6c-88dd-36cb1d599d17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StartupTask {
    type Vtable = IStartupTask_Vtbl;
    const IID: ::windows_core::GUID = <IStartupTask as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StartupTask {
    const NAME: &'static str = "Windows.ApplicationModel.StartupTask";
}
impl ::core::convert::From<StartupTask> for ::windows_core::IUnknown {
    fn from(value: StartupTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTask> for ::windows_core::IUnknown {
    fn from(value: &StartupTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StartupTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StartupTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StartupTask> for ::windows_core::IInspectable {
    fn from(value: StartupTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTask> for ::windows_core::IInspectable {
    fn from(value: &StartupTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StartupTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StartupTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StartupTask {}
unsafe impl ::core::marker::Sync for StartupTask {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StartupTaskState(pub i32);
impl StartupTaskState {
    pub const Disabled: Self = Self(0i32);
    pub const DisabledByUser: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const EnabledByPolicy: Self = Self(4i32);
}
impl ::core::marker::Copy for StartupTaskState {}
impl ::core::clone::Clone for StartupTaskState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StartupTaskState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for StartupTaskState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StartupTaskState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTaskState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StartupTaskState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.StartupTaskState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SuspendingDeferral(::windows_core::IUnknown);
impl SuspendingDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingDeferral {}
impl ::core::fmt::Debug for SuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingDeferral;{59140509-8bc9-4eb4-b636-dabdc4f46f66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
    const IID: ::windows_core::GUID = <ISuspendingDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingDeferral";
}
impl ::core::convert::From<SuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingDeferral> for ISuspendingDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingDeferral) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingDeferral> for ISuspendingDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingDeferral> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingDeferral> for &SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingDeferral> {
        ::core::convert::TryInto::<ISuspendingDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingDeferral {}
unsafe impl ::core::marker::Sync for SuspendingDeferral {}
#[repr(transparent)]
pub struct SuspendingEventArgs(::windows_core::IUnknown);
impl SuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> ::windows_core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuspendingOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SuspendingOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingEventArgs {}
impl ::core::fmt::Debug for SuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingEventArgs;{96061c05-2dba-4d08-b0bd-2b30a131c6aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISuspendingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingEventArgs";
}
impl ::core::convert::From<SuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingEventArgs> for ISuspendingEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingEventArgs> for ISuspendingEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingEventArgs> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingEventArgs> for &SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingEventArgs> {
        ::core::convert::TryInto::<ISuspendingEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingEventArgs {}
unsafe impl ::core::marker::Sync for SuspendingEventArgs {}
#[repr(transparent)]
pub struct SuspendingOperation(::windows_core::IUnknown);
impl SuspendingOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SuspendingDeferral>(result__)
        }
    }
    pub fn Deadline(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuspendingOperation {}
impl ::core::fmt::Debug for SuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SuspendingOperation;{9da4ca41-20e1-4e9b-9f65-a9f435340c3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
    const IID: ::windows_core::GUID = <ISuspendingOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingOperation";
}
impl ::core::convert::From<SuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SuspendingOperation> for ISuspendingOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SuspendingOperation> for ISuspendingOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingOperation> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISuspendingOperation> for &SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ISuspendingOperation> {
        ::core::convert::TryInto::<ISuspendingOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SuspendingOperation {}
unsafe impl ::core::marker::Sync for SuspendingOperation {}
