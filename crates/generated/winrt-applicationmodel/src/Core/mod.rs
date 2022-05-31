#[repr(transparent)]
pub struct AppListEntry(::windows_core::IUnknown);
impl AppListEntry {
    pub fn DisplayInfo(&self) -> ::windows_core::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AppDisplayInfo>(result__)
        }
    }
    pub fn LaunchAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn LaunchForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(&self, user: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows_core::Result<super::AppInfo> {
        let this = &::windows_core::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::AppInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for AppListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppListEntry {}
impl ::core::fmt::Debug for AppListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppListEntry {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.AppListEntry;{ef00f07f-2108-490a-877a-8a9f17c25fad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppListEntry {
    type Vtable = IAppListEntry_Vtbl;
    const IID: ::windows_core::GUID = <IAppListEntry as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
impl ::core::convert::From<AppListEntry> for ::windows_core::IUnknown {
    fn from(value: AppListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows_core::IUnknown {
    fn from(value: &AppListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppListEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppListEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppListEntry> for ::windows_core::IInspectable {
    fn from(value: AppListEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows_core::IInspectable {
    fn from(value: &AppListEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppListEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppListEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppListEntry {}
unsafe impl ::core::marker::Sync for AppListEntry {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: Self = Self(0i32);
    pub const NotInForeground: Self = Self(1i32);
    pub const InvalidUser: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for AppRestartFailureReason {}
impl ::core::clone::Clone for AppRestartFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppRestartFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppRestartFailureReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppRestartFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRestartFailureReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CoreApplication;
impl CoreApplication {
    pub fn Id() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Suspending<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<super::SuspendingEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Suspending)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSuspending<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSuspending)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn Resuming<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Resuming)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveResuming<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveResuming)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties() -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        })
    }
    pub fn GetCurrentView() -> ::windows_core::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn Run<'a, Param0: ::windows_core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows_core::Interface::vtable(this).Run)(::windows_core::Interface::as_raw(this), viewsource.into_param().abi()).ok() })
    }
    pub fn RunWithActivationFactories<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IGetActivationFactory>>(activationfactorycallback: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows_core::Interface::vtable(this).RunWithActivationFactories)(::windows_core::Interface::as_raw(this), activationfactorycallback.into_param().abi()).ok() })
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn BackgroundActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundActivated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveBackgroundActivated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn LeavingBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<super::LeavingBackgroundEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LeavingBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLeavingBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLeavingBackground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn EnteredBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<super::EnteredBackgroundEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnteredBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEnteredBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveEnteredBackground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows_core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows_core::Interface::vtable(this).EnablePrelaunch)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn RequestRestartAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(launcharguments: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartAsync)(::windows_core::Interface::as_raw(this), launcharguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn RequestRestartForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, launcharguments: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), launcharguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    pub fn Exit() -> ::windows_core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows_core::Interface::vtable(this).Exit)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn Exiting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Exiting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveExiting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveExiting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn UnhandledErrorDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnhandledErrorDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveUnhandledErrorDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IncrementApplicationUseCount() -> ::windows_core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows_core::Interface::vtable(this).IncrementApplicationUseCount)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn DecrementApplicationUseCount() -> ::windows_core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows_core::Interface::vtable(this).DecrementApplicationUseCount)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Views() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Views)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<CoreApplicationView>>(result__)
        })
    }
    pub fn CreateNewView<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(runtimetype: Param0, entrypoint: Param1) -> ::windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNewView)(::windows_core::Interface::as_raw(this), runtimetype.into_param().abi(), entrypoint.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn MainView() -> ::windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MainView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewFromMainView() -> ::windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNewViewFromMainView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewWithViewSource<'a, Param0: ::windows_core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows_core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNewViewWithViewSource)(::windows_core::Interface::as_raw(this), viewsource.into_param().abi(), result__.as_mut_ptr()).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplication> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplication2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplication3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplicationExit> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreApplicationUseCount> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreImmersiveApplication> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[repr(transparent)]
pub struct CoreApplicationView(::windows_core::IUnknown);
impl CoreApplicationView {
    #[cfg(feature = "winrt-ui")]
    pub fn CoreWindow(&self) -> ::windows_core::Result<::winrt_ui::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoreWindow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Core::CoreWindow>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn Activated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActivated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsMain(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsHosted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHosted)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<::winrt_ui::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Core::CoreDispatcher>(result__)
        }
    }
    pub fn IsComponent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsComponent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TitleBar(&self) -> ::windows_core::Result<CoreApplicationViewTitleBar> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TitleBar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CoreApplicationViewTitleBar>(result__)
        }
    }
    pub fn HostedViewClosing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HostedViewClosing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHostedViewClosing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHostedViewClosing)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
}
impl ::core::clone::Clone for CoreApplicationView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationView {}
impl ::core::fmt::Debug for CoreApplicationView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreApplicationView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationView;{638bb2db-451d-4661-b099-414f34ffb9f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
    const IID: ::windows_core::GUID = <ICoreApplicationView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
impl ::core::convert::From<CoreApplicationView> for ::windows_core::IUnknown {
    fn from(value: CoreApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows_core::IUnknown {
    fn from(value: &CoreApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreApplicationView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreApplicationView> for ::windows_core::IInspectable {
    fn from(value: CoreApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows_core::IInspectable {
    fn from(value: &CoreApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreApplicationView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct CoreApplicationViewTitleBar(::windows_core::IUnknown);
impl CoreApplicationViewTitleBar {
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendViewIntoTitleBar)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExtendViewIntoTitleBar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendViewIntoTitleBar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SystemOverlayLeftInset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SystemOverlayLeftInset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SystemOverlayRightInset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SystemOverlayRightInset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn LayoutMetricsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LayoutMetricsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLayoutMetricsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLayoutMetricsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsVisibleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisibleChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsVisibleChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsVisibleChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CoreApplicationViewTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreApplicationViewTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreApplicationViewTitleBar {}
impl ::core::fmt::Debug for CoreApplicationViewTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreApplicationViewTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationViewTitleBar;{006d35e3-e1f1-431b-9508-29b96926ac53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
    const IID: ::windows_core::GUID = <ICoreApplicationViewTitleBar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows_core::IUnknown {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows_core::IUnknown {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows_core::IInspectable {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows_core::IInspectable {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct HostedViewClosingEventArgs(::windows_core::IUnknown);
impl HostedViewClosingEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for HostedViewClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HostedViewClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostedViewClosingEventArgs {}
impl ::core::fmt::Debug for HostedViewClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostedViewClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.HostedViewClosingEventArgs;{d238943c-b24e-4790-acb5-3e4243c4ff87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHostedViewClosingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows_core::IUnknown {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows_core::IInspectable {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HostedViewClosingEventArgs {}
unsafe impl ::core::marker::Sync for HostedViewClosingEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppListEntry {
    type Vtable = IAppListEntry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef00f07f_2108_490a_877a_8a9f17c25fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppListEntry2 {
    type Vtable = IAppListEntry2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0a618ad_bf35_42ac_ac06_86eeeb41d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppListEntry3 {
    type Vtable = IAppListEntry3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6099f28d_fc32_470a_bc69_4b061a76ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub LaunchForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    LaunchForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppListEntry4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppListEntry4 {
    type Vtable = IAppListEntry4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a131ed2_56f5_487c_8697_5166f3b33da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplication {
    type Vtable = ICoreApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0aacf7a4_5e1d_49df_8034_fb6a68bc5ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
    pub GetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RunWithActivationFactories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationfactorycallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplication2 {
    type Vtable = ICoreApplication2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x998681fb_1ab6_4b7f_be4a_9a0645224c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-applicationmodel")]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    BackgroundActivated: usize,
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplication3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplication3 {
    type Vtable = ICoreApplication3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeec0d39_598b_4507_8a67_772632580a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationExit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationExit {
    type Vtable = ICoreApplicationExit_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf86461d_261e_4b72_9acd_44ed2ace6a29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationExit_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Exiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveExiting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICoreApplicationUnhandledError(::windows_core::IUnknown);
impl ICoreApplicationUnhandledError {
    pub fn UnhandledErrorDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnhandledErrorDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnhandledErrorDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows_core::IUnknown {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows_core::IUnknown {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows_core::IInspectable {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows_core::IInspectable {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICoreApplicationUnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreApplicationUnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreApplicationUnhandledError {}
impl ::core::fmt::Debug for ICoreApplicationUnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreApplicationUnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICoreApplicationUnhandledError {
    type Vtable = ICoreApplicationUnhandledError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0e24ab0_dd09_42e1_b0bc_e0e131f78d7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUnhandledError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnhandledErrorDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationUseCount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationUseCount {
    type Vtable = ICoreApplicationUseCount_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x518dc408_c077_475b_809e_0bc0c57e4b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IncrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DecrementApplicationUseCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationView {
    type Vtable = ICoreApplicationView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x638bb2db_451d_4661_b099_414f34ffb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CoreWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CoreWindow: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsHosted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationView2 {
    type Vtable = ICoreApplicationView2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68eb7adf_917f_48eb_9aeb_7de53e086ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationView3 {
    type Vtable = ICoreApplicationView3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07ebe1b3_a4cf_4550_ab70_b07e85330bc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHostedViewClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationView5 {
    type Vtable = ICoreApplicationView5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bc095a8_8ef0_446d_9e60_3a3e0428c671);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationView6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationView6 {
    type Vtable = ICoreApplicationView6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc119d49a_0679_49ba_803f_b79c5cf34cca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-system")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreApplicationViewTitleBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x006d35e3_e1f1_431b_9508_29b96926ac53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ExtendViewIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SystemOverlayLeftInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SystemOverlayRightInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub LayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLayoutMetricsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsVisibleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreImmersiveApplication {
    type Vtable = ICoreImmersiveApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ada0e3e_e4a2_4123_b451_dc96bf800419);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Views: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Views: usize,
    pub CreateNewView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runtimetype: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, entrypoint: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreImmersiveApplication2 {
    type Vtable = ICoreImmersiveApplication2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x828e1e36_e9e3_4cfc_9b66_48b78ea9bb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateNewViewFromMainView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreImmersiveApplication3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreImmersiveApplication3 {
    type Vtable = ICoreImmersiveApplication3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a05b2f_ee0d_41e5_8314_cf10c91bf0af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateNewViewWithViewSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewsource: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFrameworkView(::windows_core::IUnknown);
impl IFrameworkView {
    pub fn Initialize<'a, Param0: ::windows_core::IntoParam<'a, CoreApplicationView>>(&self, applicationview: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Initialize)(::windows_core::Interface::as_raw(this), applicationview.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetWindow<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Core::CoreWindow>>(&self, window: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi()).ok() }
    }
    pub fn Load<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, entrypoint: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Load)(::windows_core::Interface::as_raw(this), entrypoint.into_param().abi()).ok() }
    }
    pub fn Run(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Run)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uninitialize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Uninitialize)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IFrameworkView> for ::windows_core::IUnknown {
    fn from(value: IFrameworkView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows_core::IUnknown {
    fn from(value: &IFrameworkView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFrameworkView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFrameworkView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFrameworkView> for ::windows_core::IInspectable {
    fn from(value: IFrameworkView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows_core::IInspectable {
    fn from(value: &IFrameworkView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFrameworkView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFrameworkView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFrameworkView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkView {}
impl ::core::fmt::Debug for IFrameworkView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFrameworkView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{faab5cd0-8924-45ac-ad0f-a08fae5d0324}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFrameworkView {
    type Vtable = IFrameworkView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaab5cd0_8924_45ac_ad0f_a08fae5d0324);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationview: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub SetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetWindow: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entrypoint: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFrameworkViewSource(::windows_core::IUnknown);
impl IFrameworkViewSource {
    pub fn CreateView(&self) -> ::windows_core::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IFrameworkView>(result__)
        }
    }
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows_core::IUnknown {
    fn from(value: IFrameworkViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows_core::IUnknown {
    fn from(value: &IFrameworkViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFrameworkViewSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows_core::IInspectable {
    fn from(value: IFrameworkViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows_core::IInspectable {
    fn from(value: &IFrameworkViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFrameworkViewSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFrameworkViewSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFrameworkViewSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFrameworkViewSource {}
impl ::core::fmt::Debug for IFrameworkViewSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFrameworkViewSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{cd770614-65c4-426c-9494-34fc43554862}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd770614_65c4_426c_9494_34fc43554862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostedViewClosingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd238943c_b24e_4790_acb5_3e4243c4ff87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostedViewClosingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnhandledError {
    type Vtable = IUnhandledError_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9459b726_53b5_4686_9eaf_fa8162dc3980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledError_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Propagate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnhandledErrorDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UnhandledError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct UnhandledError(::windows_core::IUnknown);
impl UnhandledError {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Propagate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Propagate)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UnhandledError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledError {}
impl ::core::fmt::Debug for UnhandledError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnhandledError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledError;{9459b726-53b5-4686-9eaf-fa8162dc3980})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UnhandledError {
    type Vtable = IUnhandledError_Vtbl;
    const IID: ::windows_core::GUID = <IUnhandledError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
impl ::core::convert::From<UnhandledError> for ::windows_core::IUnknown {
    fn from(value: UnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows_core::IUnknown {
    fn from(value: &UnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnhandledError> for ::windows_core::IInspectable {
    fn from(value: UnhandledError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows_core::IInspectable {
    fn from(value: &UnhandledError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnhandledError {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnhandledError {}
unsafe impl ::core::marker::Sync for UnhandledError {}
#[repr(transparent)]
pub struct UnhandledErrorDetectedEventArgs(::windows_core::IUnknown);
impl UnhandledErrorDetectedEventArgs {
    pub fn UnhandledError(&self) -> ::windows_core::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnhandledError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UnhandledError>(result__)
        }
    }
}
impl ::core::clone::Clone for UnhandledErrorDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnhandledErrorDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnhandledErrorDetectedEventArgs {}
impl ::core::fmt::Debug for UnhandledErrorDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnhandledErrorDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs;{679ab78b-b336-4822-ac40-0d750f0b7a2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUnhandledErrorDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnhandledErrorDetectedEventArgs {}
unsafe impl ::core::marker::Sync for UnhandledErrorDetectedEventArgs {}
