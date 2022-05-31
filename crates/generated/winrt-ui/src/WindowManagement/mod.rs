#[cfg(feature = "Preview")]
pub mod Preview;
#[repr(transparent)]
pub struct AppWindow(::windows_core::IUnknown);
impl AppWindow {
    pub fn Content(&self) -> ::windows_core::Result<super::UIContentRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIContentRoot>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn Frame(&self) -> ::windows_core::Result<AppWindowFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowFrame>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PersistedStateId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PersistedStateId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetPersistedStateId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPersistedStateId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Presenter(&self) -> ::windows_core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Presenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresenter>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TitleBar(&self) -> ::windows_core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TitleBar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowTitleBar>(result__)
        }
    }
    pub fn UIContext(&self) -> ::windows_core::Result<super::UIContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UIContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIContext>(result__)
        }
    }
    pub fn WindowingEnvironment(&self) -> ::windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowingEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowingEnvironment>(result__)
        }
    }
    pub fn CloseAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloseAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetPlacement(&self) -> ::windows_core::Result<AppWindowPlacement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetPlacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPlacement>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDisplayRegions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<DisplayRegion>>(result__)
        }
    }
    pub fn RequestMoveToDisplayRegion<'a, Param0: ::windows_core::IntoParam<'a, DisplayRegion>>(&self, displayregion: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveToDisplayRegion)(::windows_core::Interface::as_raw(this), displayregion.into_param().abi()).ok() }
    }
    pub fn RequestMoveAdjacentToCurrentView(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveAdjacentToCurrentView)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RequestMoveAdjacentToWindow<'a, Param0: ::windows_core::IntoParam<'a, AppWindow>>(&self, anchorwindow: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveAdjacentToWindow)(::windows_core::Interface::as_raw(this), anchorwindow.into_param().abi()).ok() }
    }
    pub fn RequestMoveRelativeToWindowContent<'a, Param0: ::windows_core::IntoParam<'a, AppWindow>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, anchorwindow: Param0, contentoffset: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveRelativeToWindowContent)(::windows_core::Interface::as_raw(this), anchorwindow.into_param().abi(), contentoffset.into_param().abi()).ok() }
    }
    pub fn RequestMoveRelativeToCurrentViewContent<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, contentoffset: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveRelativeToCurrentViewContent)(::windows_core::Interface::as_raw(this), contentoffset.into_param().abi()).ok() }
    }
    pub fn RequestMoveRelativeToDisplayRegion<'a, Param0: ::windows_core::IntoParam<'a, DisplayRegion>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, displayregion: Param0, displayregionoffset: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestMoveRelativeToDisplayRegion)(::windows_core::Interface::as_raw(this), displayregion.into_param().abi(), displayregionoffset.into_param().abi()).ok() }
    }
    pub fn RequestSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Size>>(&self, framesize: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestSize)(::windows_core::Interface::as_raw(this), framesize.into_param().abi()).ok() }
    }
    pub fn TryShowAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryShowAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Changed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CloseRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CloseRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCloseRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCloseRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TryCreateAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppWindow>> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppWindow>>(result__)
        })
    }
    pub fn ClearAllPersistedState() -> ::windows_core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ClearAllPersistedState)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ClearPersistedState<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(key: Param0) -> ::windows_core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ClearPersistedState)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() })
    }
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppWindow, IAppWindowStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindow {}
impl ::core::fmt::Debug for AppWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindow;{663014a6-b75e-5dbd-995c-f0117fa3fb61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindow as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindow";
}
impl ::core::convert::From<AppWindow> for ::windows_core::IUnknown {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows_core::IUnknown {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindow> for ::windows_core::IInspectable {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows_core::IInspectable {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindow {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(::windows_core::IUnknown);
impl AppWindowChangedEventArgs {
    pub fn DidAvailableWindowPresentationsChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidAvailableWindowPresentationsChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidDisplayRegionsChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidDisplayRegionsChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidFrameChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidFrameChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidSizeChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidSizeChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidTitleBarChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidTitleBarChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidVisibilityChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidWindowingEnvironmentChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidWindowingEnvironmentChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DidWindowPresentationChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).DidWindowPresentationChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowChangedEventArgs {}
impl ::core::fmt::Debug for AppWindowChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowChangedEventArgs;{1de1f3be-a655-55ad-b2b6-eb240f880356})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowChangedEventArgs";
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
pub struct AppWindowCloseRequestedEventArgs(::windows_core::IUnknown);
impl AppWindowCloseRequestedEventArgs {
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
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowCloseRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowCloseRequestedEventArgs {}
impl ::core::fmt::Debug for AppWindowCloseRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowCloseRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowCloseRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs;{e9ff01da-e7a2-57a8-8b5e-39c4003afdbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowCloseRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
}
impl ::core::convert::From<AppWindowCloseRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppWindowCloseRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowCloseRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppWindowCloseRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowCloseRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppWindowCloseRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowCloseRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppWindowCloseRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowCloseRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowCloseRequestedEventArgs {}
#[repr(transparent)]
pub struct AppWindowClosedEventArgs(::windows_core::IUnknown);
impl AppWindowClosedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<AppWindowClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowClosedReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowClosedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosedEventArgs {}
impl ::core::fmt::Debug for AppWindowClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowClosedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowClosedEventArgs;{cc7df816-9520-5a06-821e-456ad8b358aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowClosedEventArgs";
}
impl ::core::convert::From<AppWindowClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppWindowClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppWindowClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppWindowClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppWindowClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: Self = Self(0i32);
    pub const AppInitiated: Self = Self(1i32);
    pub const UserInitiated: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowClosedReason {}
impl ::core::clone::Clone for AppWindowClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppWindowClosedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppWindowClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowClosedReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowClosedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppWindowFrame(::windows_core::IUnknown);
impl AppWindowFrame {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub fn DragRegionVisuals(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::Composition::IVisualElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragRegionVisuals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::Composition::IVisualElement>>(result__)
        }
    }
    pub fn GetFrameStyle(&self) -> ::windows_core::Result<AppWindowFrameStyle> {
        let this = &::windows_core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowFrameStyle>::zeroed();
            (::windows_core::Interface::vtable(this).GetFrameStyle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowFrameStyle>(result__)
        }
    }
    pub fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFrameStyle)(::windows_core::Interface::as_raw(this), framestyle).ok() }
    }
}
impl ::core::clone::Clone for AppWindowFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowFrame {}
impl ::core::fmt::Debug for AppWindowFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowFrame;{9ee22601-7e5d-52af-846b-01dc6c296567})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowFrame {
    type Vtable = IAppWindowFrame_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowFrame";
}
impl ::core::convert::From<AppWindowFrame> for ::windows_core::IUnknown {
    fn from(value: AppWindowFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowFrame> for ::windows_core::IUnknown {
    fn from(value: &AppWindowFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowFrame> for ::windows_core::IInspectable {
    fn from(value: AppWindowFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowFrame> for ::windows_core::IInspectable {
    fn from(value: &AppWindowFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowFrame {}
unsafe impl ::core::marker::Sync for AppWindowFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: Self = Self(0i32);
    pub const NoFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowFrameStyle {}
impl ::core::clone::Clone for AppWindowFrameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowFrameStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppWindowFrameStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppWindowFrameStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrameStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowFrameStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowFrameStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppWindowPlacement(::windows_core::IUnknown);
impl AppWindowPlacement {
    pub fn DisplayRegion(&self) -> ::windows_core::Result<DisplayRegion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayRegion)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayRegion>(result__)
        }
    }
    pub fn Offset(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPlacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPlacement {}
impl ::core::fmt::Debug for AppWindowPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPlacement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowPlacement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPlacement;{03dc815e-e7a9-5857-9c03-7d670594410e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowPlacement {
    type Vtable = IAppWindowPlacement_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowPlacement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPlacement";
}
impl ::core::convert::From<AppWindowPlacement> for ::windows_core::IUnknown {
    fn from(value: AppWindowPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPlacement> for ::windows_core::IUnknown {
    fn from(value: &AppWindowPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowPlacement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowPlacement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPlacement> for ::windows_core::IInspectable {
    fn from(value: AppWindowPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPlacement> for ::windows_core::IInspectable {
    fn from(value: &AppWindowPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowPlacement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowPlacement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPlacement {}
unsafe impl ::core::marker::Sync for AppWindowPlacement {}
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(::windows_core::IUnknown);
impl AppWindowPresentationConfiguration {
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresentationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresentationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresentationConfiguration {}
impl ::core::fmt::Debug for AppWindowPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowPresentationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresentationConfiguration;{b5a43ee3-df33-5e67-bd31-1072457300df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowPresentationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
}
impl ::core::convert::From<AppWindowPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: AppWindowPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &AppWindowPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: AppWindowPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &AppWindowPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPresentationConfiguration {}
unsafe impl ::core::marker::Sync for AppWindowPresentationConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowPresentationKind {}
impl ::core::clone::Clone for AppWindowPresentationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowPresentationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppWindowPresentationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppWindowPresentationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowPresentationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowPresentationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppWindowPresenter(::windows_core::IUnknown);
impl AppWindowPresenter {
    pub fn GetConfiguration(&self) -> ::windows_core::Result<AppWindowPresentationConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetConfiguration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresentationConfiguration>(result__)
        }
    }
    pub fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPresentationSupported)(::windows_core::Interface::as_raw(this), presentationkind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RequestPresentation<'a, Param0: ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration>>(&self, configuration: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPresentation)(::windows_core::Interface::as_raw(this), configuration.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPresentationByKind)(::windows_core::Interface::as_raw(this), presentationkind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresenter {}
impl ::core::fmt::Debug for AppWindowPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresenter;{5ae9ed73-e1fd-5317-ad78-5a3ed271bbde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowPresenter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresenter";
}
impl ::core::convert::From<AppWindowPresenter> for ::windows_core::IUnknown {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows_core::IUnknown {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPresenter> for ::windows_core::IInspectable {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows_core::IInspectable {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[repr(transparent)]
pub struct AppWindowTitleBar(::windows_core::IUnknown);
impl AppWindowTitleBar {
    pub fn BackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonBackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonHoverBackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonHoverBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonHoverBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonHoverBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonHoverForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonHoverForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonHoverForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonHoverForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonInactiveBackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonInactiveBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonInactiveBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonInactiveForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonInactiveForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonInactiveForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonInactiveForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonPressedBackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressedBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonPressedBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonPressedBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ButtonPressedForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressedForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetButtonPressedForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetButtonPressedForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendsContentIntoTitleBar)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn InactiveBackgroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InactiveBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetInactiveBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInactiveBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn InactiveForegroundColor(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InactiveForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::Color>>(result__)
        }
    }
    pub fn SetInactiveForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInactiveForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTitleBarOcclusions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTitleBarOcclusions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>(result__)
        }
    }
    pub fn GetPreferredVisibility(&self) -> ::windows_core::Result<AppWindowTitleBarVisibility> {
        let this = &::windows_core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowTitleBarVisibility>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreferredVisibility)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowTitleBarVisibility>(result__)
        }
    }
    pub fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredVisibility)(::windows_core::Interface::as_raw(this), visibilitymode).ok() }
    }
}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBar {}
impl ::core::fmt::Debug for AppWindowTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBar;{6e932c84-f644-541d-a2d7-0c262437842d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowTitleBar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBar";
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows_core::IUnknown {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows_core::IUnknown {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows_core::IInspectable {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows_core::IInspectable {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowTitleBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(::windows_core::IUnknown);
impl AppWindowTitleBarOcclusion {
    pub fn OccludingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).OccludingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBarOcclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBarOcclusion {}
impl ::core::fmt::Debug for AppWindowTitleBarOcclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarOcclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowTitleBarOcclusion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBarOcclusion;{fea3cffd-2ccf-5fc3-aeae-f843876bf37e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusion_Vtbl;
    const IID: ::windows_core::GUID = <IAppWindowTitleBarOcclusion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
}
impl ::core::convert::From<AppWindowTitleBarOcclusion> for ::windows_core::IUnknown {
    fn from(value: AppWindowTitleBarOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBarOcclusion> for ::windows_core::IUnknown {
    fn from(value: &AppWindowTitleBarOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowTitleBarOcclusion> for ::windows_core::IInspectable {
    fn from(value: AppWindowTitleBarOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBarOcclusion> for ::windows_core::IInspectable {
    fn from(value: &AppWindowTitleBarOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBarOcclusion {}
unsafe impl ::core::marker::Sync for AppWindowTitleBarOcclusion {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: Self = Self(0i32);
    pub const AlwaysHidden: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowTitleBarVisibility {}
impl ::core::clone::Clone for AppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowTitleBarVisibility {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppWindowTitleBarVisibility {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppWindowTitleBarVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarVisibility").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppWindowTitleBarVisibility {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowTitleBarVisibility;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct CompactOverlayPresentationConfiguration(::windows_core::IUnknown);
impl CompactOverlayPresentationConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompactOverlayPresentationConfiguration, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresentationKind> {
        let this = &::windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresentationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for CompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompactOverlayPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompactOverlayPresentationConfiguration {}
impl ::core::fmt::Debug for CompactOverlayPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlayPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompactOverlayPresentationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration;{a7e5750f-5730-56c6-8e1f-d63ff4d7980d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <ICompactOverlayPresentationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for &CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompactOverlayPresentationConfiguration {}
unsafe impl ::core::marker::Sync for CompactOverlayPresentationConfiguration {}
#[repr(transparent)]
pub struct DefaultPresentationConfiguration(::windows_core::IUnknown);
impl DefaultPresentationConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DefaultPresentationConfiguration, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresentationKind> {
        let this = &::windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresentationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for DefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DefaultPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultPresentationConfiguration {}
impl ::core::fmt::Debug for DefaultPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DefaultPresentationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DefaultPresentationConfiguration;{d8c2b53b-2168-5703-a853-d525589fe2b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IDefaultPresentationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.DefaultPresentationConfiguration";
}
impl ::core::convert::From<DefaultPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DefaultPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DefaultPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for &DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for DefaultPresentationConfiguration {}
unsafe impl ::core::marker::Sync for DefaultPresentationConfiguration {}
#[repr(transparent)]
pub struct DisplayRegion(::windows_core::IUnknown);
impl DisplayRegion {
    pub fn DisplayMonitorDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayMonitorDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn WorkAreaOffset(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).WorkAreaOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn WorkAreaSize(&self) -> ::windows_core::Result<::winrt_foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).WorkAreaSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        }
    }
    pub fn WindowingEnvironment(&self) -> ::windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowingEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowingEnvironment>(result__)
        }
    }
    pub fn Changed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayRegion, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DisplayRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayRegion {}
impl ::core::fmt::Debug for DisplayRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayRegion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DisplayRegion;{db50c3a2-4094-5f47-8cb1-ea01ddafaa94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayRegion {
    type Vtable = IDisplayRegion_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayRegion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.DisplayRegion";
}
impl ::core::convert::From<DisplayRegion> for ::windows_core::IUnknown {
    fn from(value: DisplayRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRegion> for ::windows_core::IUnknown {
    fn from(value: &DisplayRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayRegion> for ::windows_core::IInspectable {
    fn from(value: DisplayRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRegion> for ::windows_core::IInspectable {
    fn from(value: &DisplayRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayRegion {}
unsafe impl ::core::marker::Sync for DisplayRegion {}
#[repr(transparent)]
pub struct FullScreenPresentationConfiguration(::windows_core::IUnknown);
impl FullScreenPresentationConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FullScreenPresentationConfiguration, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresentationKind> {
        let this = &::windows_core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppWindowPresentationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
    pub fn IsExclusive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsExclusive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsExclusive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsExclusive)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullScreenPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullScreenPresentationConfiguration {}
impl ::core::fmt::Debug for FullScreenPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FullScreenPresentationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.FullScreenPresentationConfiguration;{43d3dcd8-d2a8-503d-a626-15533d6d5f62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IFullScreenPresentationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, AppWindowPresentationConfiguration> for &FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for FullScreenPresentationConfiguration {}
unsafe impl ::core::marker::Sync for FullScreenPresentationConfiguration {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindow {
    type Vtable = IAppWindow_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x663014a6_b75e_5dbd_995c_f0117fa3fb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Presenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CloseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub RequestMoveToDisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestMoveAdjacentToCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestMoveAdjacentToWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestMoveRelativeToWindowContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: ::windows_core::RawPtr, contentoffset: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub RequestMoveRelativeToCurrentViewContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentoffset: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub RequestMoveRelativeToDisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows_core::RawPtr, displayregionoffset: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub RequestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesize: ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub TryShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1de1f3be_a655_55ad_b2b6_eb240f880356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DidAvailableWindowPresentationsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidDisplayRegionsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidFrameChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidTitleBarChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidWindowingEnvironmentChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DidWindowPresentationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowCloseRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9ff01da_e7a2_57a8_8b5e_39c4003afdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowCloseRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc7df816_9520_5a06_821e_456ad8b358aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowClosedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowFrame {
    type Vtable = IAppWindowFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ee22601_7e5d_52af_846b_01dc6c296567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub DragRegionVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))]
    DragRegionVisuals: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrameStyle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowFrameStyle {
    type Vtable = IAppWindowFrameStyle_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac412946_e1ac_5230_944a_c60873dcf4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrameStyle_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetFrameStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowFrameStyle) -> ::windows_core::HRESULT,
    pub SetFrameStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framestyle: AppWindowFrameStyle) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPlacement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPlacement {
    type Vtable = IAppWindowPlacement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03dc815e_e7a9_5857_9c03_7d670594410e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPlacement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5a43ee3_df33_5e67_bd31_1072457300df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowPresentationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfigurationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPresentationConfigurationFactory {
    type Vtable = IAppWindowPresentationConfigurationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd3606a6_7875_5de8_84ff_6351ee13dd0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfigurationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ae9ed73_e1fd_5317_ad78_5a3ed271bbde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RequestPresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub RequestPresentationByKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff1f3ea3_b769_50ef_9873_108cd0e89746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClearAllPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e932c84_f644_541d_a2d7_0c262437842d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTitleBarOcclusions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTitleBarOcclusions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarOcclusion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfea3cffd_2ccf_5fc3_aeae_f843876bf37e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarOcclusion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OccludingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarVisibility(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBarVisibility {
    type Vtable = IAppWindowTitleBarVisibility_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa215a4e3_6e7e_5651_8c3b_624819528154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarVisibility_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetPreferredVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowTitleBarVisibility) -> ::windows_core::HRESULT,
    pub SetPreferredVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibilitymode: AppWindowTitleBarVisibility) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresentationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7e5750f_5730_56c6_8e1f_d63ff4d7980d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresentationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDefaultPresentationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8c2b53b_2168_5703_a853_d525589fe2b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultPresentationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayRegion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayRegion {
    type Vtable = IDisplayRegion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb50c3a2_4094_5f47_8cb1_ea01ddafaa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayRegion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DisplayMonitorDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub WorkAreaOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub WorkAreaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresentationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43d3dcd8_d2a8_503d_a626_15533d6d5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresentationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowServicesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowServicesStatics {
    type Vtable = IWindowServicesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcff4d519_50a6_5c64_97f6_c2d96add7f42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowServicesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllTopLevelWindowIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllTopLevelWindowIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowingEnvironment {
    type Vtable = IWindowingEnvironment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x264363c0_2a49_5417_b3ae_48a71c63a3bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowingEnvironmentKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDisplayRegions: usize,
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff2a5b7f_f183_5c66_99b2_429082069299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4160cfc6_023d_5e9a_b431_350e67dc978a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e5b5473_beff_5e53_9316_7e775fe568b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowingEnvironmentStatics {
    type Vtable = IWindowingEnvironmentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x874e9fb7_c642_55ab_8aa2_162f734a9a72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: WindowingEnvironmentKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllWithKind: usize,
}
pub struct WindowServices;
impl WindowServices {
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllTopLevelWindowIds() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::WindowId>> {
        Self::IWindowServicesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllTopLevelWindowIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::WindowId>>(result__)
        })
    }
    pub fn IWindowServicesStatics<R, F: FnOnce(&IWindowServicesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowServices, IWindowServicesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WindowServices {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowServices";
}
#[repr(transparent)]
pub struct WindowingEnvironment(::windows_core::IUnknown);
impl WindowingEnvironment {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WindowingEnvironmentKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WindowingEnvironmentKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowingEnvironmentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDisplayRegions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<DisplayRegion>>(result__)
        }
    }
    pub fn Changed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<WindowingEnvironment>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithKind(kind: WindowingEnvironmentKind) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllWithKind)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<WindowingEnvironment>>(result__)
        })
    }
    pub fn IWindowingEnvironmentStatics<R, F: FnOnce(&IWindowingEnvironmentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowingEnvironment, IWindowingEnvironmentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowingEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironment {}
impl ::core::fmt::Debug for WindowingEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowingEnvironment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironment;{264363c0-2a49-5417-b3ae-48a71c63a3bd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowingEnvironment {
    type Vtable = IWindowingEnvironment_Vtbl;
    const IID: ::windows_core::GUID = <IWindowingEnvironment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironment";
}
impl ::core::convert::From<WindowingEnvironment> for ::windows_core::IUnknown {
    fn from(value: WindowingEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironment> for ::windows_core::IUnknown {
    fn from(value: &WindowingEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowingEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowingEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironment> for ::windows_core::IInspectable {
    fn from(value: WindowingEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironment> for ::windows_core::IInspectable {
    fn from(value: &WindowingEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowingEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowingEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironment {}
unsafe impl ::core::marker::Sync for WindowingEnvironment {}
#[repr(transparent)]
pub struct WindowingEnvironmentAddedEventArgs(::windows_core::IUnknown);
impl WindowingEnvironmentAddedEventArgs {
    pub fn WindowingEnvironment(&self) -> ::windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowingEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowingEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for WindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentAddedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowingEnvironmentAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs;{ff2a5b7f-f183-5c66-99b2-429082069299})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWindowingEnvironmentAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WindowingEnvironmentAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WindowingEnvironmentAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WindowingEnvironmentAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WindowingEnvironmentAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentAddedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentAddedEventArgs {}
#[repr(transparent)]
pub struct WindowingEnvironmentChangedEventArgs(::windows_core::IUnknown);
impl WindowingEnvironmentChangedEventArgs {}
impl ::core::clone::Clone for WindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentChangedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowingEnvironmentChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs;{4160cfc6-023d-5e9a-b431-350e67dc978a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWindowingEnvironmentChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WindowingEnvironmentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WindowingEnvironmentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WindowingEnvironmentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WindowingEnvironmentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: Self = Self(0i32);
    pub const Overlapped: Self = Self(1i32);
    pub const Tiled: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowingEnvironmentKind {}
impl ::core::clone::Clone for WindowingEnvironmentKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowingEnvironmentKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WindowingEnvironmentKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowingEnvironmentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowingEnvironmentKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.WindowingEnvironmentKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WindowingEnvironmentRemovedEventArgs(::windows_core::IUnknown);
impl WindowingEnvironmentRemovedEventArgs {
    pub fn WindowingEnvironment(&self) -> ::windows_core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowingEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WindowingEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for WindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentRemovedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowingEnvironmentRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs;{2e5b5473-beff-5e53-9316-7e775fe568b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWindowingEnvironmentRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WindowingEnvironmentRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WindowingEnvironmentRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WindowingEnvironmentRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WindowingEnvironmentRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentRemovedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentRemovedEventArgs {}
