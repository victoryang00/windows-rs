#[cfg(feature = "Interop")]
pub mod Interop;
#[repr(transparent)]
pub struct IWebViewControl(::windows_core::IUnknown);
impl IWebViewControl {
    pub fn Source(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    pub fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGoBack)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanGoForward(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGoForward)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetDefaultBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Settings(&self) -> ::windows_core::Result<WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Settings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlSettings>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DeferredPermissionRequests(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeferredPermissionRequests)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>(result__)
        }
    }
    pub fn GoForward(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GoForward)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoBack(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GoBack)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Refresh(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Refresh)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Navigate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Navigate)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    pub fn NavigateToString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToString)(::windows_core::Interface::as_raw(this), text.into_param().abi()).ok() }
    }
    pub fn NavigateToLocalStreamUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, super::IUriToStreamResolver>>(&self, source: Param0, streamresolver: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(::windows_core::Interface::as_raw(this), source.into_param().abi(), streamresolver.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn NavigateWithHttpRequestMessage<'a, Param0: ::windows_core::IntoParam<'a, super::Http::HttpRequestMessage>>(&self, requestmessage: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::windows_core::Interface::as_raw(this), requestmessage.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InvokeScriptAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, scriptname: Param0, arguments: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InvokeScriptAsync)(::windows_core::Interface::as_raw(this), scriptname.into_param().abi(), arguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn CapturePreviewToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-applicationmodel")]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::DataTransfer::DataPackage>>(result__)
        }
    }
    pub fn BuildLocalStreamUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, contentidentifier: Param0, relativepath: Param1) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildLocalStreamUri)(::windows_core::Interface::as_raw(this), contentidentifier.into_param().abi(), relativepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(::windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    pub fn NavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentLoading)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FrameNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FrameContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameContentLoading)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FrameDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn FrameNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFrameNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ScriptNotify<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScriptNotify)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScriptNotify<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScriptNotify)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn LongRunningScriptDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LongRunningScriptDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLongRunningScriptDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UnsafeContentWarningDisplaying<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnsafeContentWarningDisplaying<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UnviewableContentIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnviewableContentIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnviewableContentIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PermissionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePermissionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePermissionRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn UnsupportedUriSchemeIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnsupportedUriSchemeIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn NewWindowRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NewWindowRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNewWindowRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNewWindowRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ContainsFullScreenElementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContainsFullScreenElementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn WebResourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WebResourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveWebResourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWebResourceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebViewControl> for ::windows_core::IUnknown {
    fn from(value: IWebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl> for ::windows_core::IUnknown {
    fn from(value: &IWebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebViewControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebViewControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebViewControl> for ::windows_core::IInspectable {
    fn from(value: IWebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl> for ::windows_core::IInspectable {
    fn from(value: &IWebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebViewControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebViewControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebViewControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebViewControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebViewControl {}
impl ::core::fmt::Debug for IWebViewControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebViewControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebViewControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3f921316-bc70-4bda-9136-c94370899fab}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebViewControl {
    type Vtable = IWebViewControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f921316_bc70_4bda_9136_c94370899fab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetDefaultBackgroundColor: usize,
    #[cfg(feature = "winrt-ui")]
    pub DefaultBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    DefaultBackgroundColor: usize,
    pub ContainsFullScreenElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub DeferredPermissionRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DeferredPermissionRequests: usize,
    pub GoForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GoBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NavigateToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows_core::RawPtr, streamresolver: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-web")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestmessage: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    NavigateWithHttpRequestMessage: usize,
    #[cfg(feature = "winrt-foundation")]
    pub InvokeScriptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, arguments: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    InvokeScriptAsync: usize,
    #[cfg(feature = "winrt-storage")]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(feature = "winrt-applicationmodel")]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-applicationmodel"))]
    CaptureSelectedContentToDataPackageAsync: usize,
    pub BuildLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidentifier: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, relativepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferredPermissionRequestById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub FrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub LongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub WebResourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveWebResourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebViewControl2(::windows_core::IUnknown);
impl IWebViewControl2 {
    pub fn AddInitializeScript<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, script: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddInitializeScript)(::windows_core::Interface::as_raw(this), script.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebViewControl2> for ::windows_core::IUnknown {
    fn from(value: IWebViewControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl2> for ::windows_core::IUnknown {
    fn from(value: &IWebViewControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebViewControl2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebViewControl2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebViewControl2> for ::windows_core::IInspectable {
    fn from(value: IWebViewControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl2> for ::windows_core::IInspectable {
    fn from(value: &IWebViewControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebViewControl2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebViewControl2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebViewControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebViewControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebViewControl2 {}
impl ::core::fmt::Debug for IWebViewControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebViewControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebViewControl2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4d3c06f9-c8df-41cc-8bd5-2a947b204503}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebViewControl2 {
    type Vtable = IWebViewControl2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d3c06f9_c8df_41cc_8bd5_2a947b204503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddInitializeScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, script: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlContentLoadingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a3fccb2_b9bb_404b_a22b_66dccd1250c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlContentLoadingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDOMContentLoadedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe8bc008_9541_4545_9ff2_2df585b29f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDOMContentLoadedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDeferredPermissionRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ce349e0_d759_445c_9926_8995298f152b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDeferredPermissionRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PermissionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a6e5bba_98b4_45bc_bbeb_0f69ce49c599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExecutionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StopPageScriptExecution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStopPageScriptExecution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNavigationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20409918_4a15_4c46_a55d_f79edb0bde8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSuccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WebErrorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNavigationStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c9057c5_0a08_41c7_863b_71e3a9549137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df44bbb_a124_46d5_a083_d02cacdff5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlNewWindowRequestedEventArgs2 {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb53c5ca6_2aae_4bfc_92b9_c30e92b48098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlPermissionRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5bc836c_f22f_40e2_95b2_7729f840eb7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PermissionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionState) -> ::windows_core::HRESULT,
    pub Defer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlPermissionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27204d51_2488_4cc5_968e_0a771e59c147);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PermissionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlScriptNotifyEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x491de57b_6f49_41bb_b591_51b85b817037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlScriptNotifyEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9967fbf_5e98_4cfd_8cce_27b0911e3de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetIsJavaScriptEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsJavaScriptEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsIndexedDBEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsIndexedDBEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3b81944_e4fc_43dc_94ca_f980f30bc51d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a9680db_88f2_4e20_b693_b4e2df4aa581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlWebResourceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44d6524d_55a4_4d8b_891c_931d8e25d42e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlWebResourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-web")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    Request: usize,
    #[cfg(feature = "winrt-web")]
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    SetResponse: usize,
    #[cfg(feature = "winrt-web")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-web"))]
    Response: usize,
}
#[repr(transparent)]
pub struct WebViewControlContentLoadingEventArgs(::windows_core::IUnknown);
impl WebViewControlContentLoadingEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlContentLoadingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlContentLoadingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlContentLoadingEventArgs {}
impl ::core::fmt::Debug for WebViewControlContentLoadingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlContentLoadingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlContentLoadingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlContentLoadingEventArgs;{9a3fccb2-b9bb-404b-a22b-66dccd1250c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlContentLoadingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlContentLoadingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlContentLoadingEventArgs";
}
impl ::core::convert::From<WebViewControlContentLoadingEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlContentLoadingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlContentLoadingEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlContentLoadingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlDOMContentLoadedEventArgs(::windows_core::IUnknown);
impl WebViewControlDOMContentLoadedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlDOMContentLoadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlDOMContentLoadedEventArgs {}
impl ::core::fmt::Debug for WebViewControlDOMContentLoadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlDOMContentLoadedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlDOMContentLoadedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs;{be8bc008-9541-4545-9ff2-2df585b29f7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlDOMContentLoadedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlDOMContentLoadedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
}
impl ::core::convert::From<WebViewControlDOMContentLoadedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlDOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDOMContentLoadedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlDOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlDOMContentLoadedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlDOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDOMContentLoadedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlDOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlDeferredPermissionRequest(::windows_core::IUnknown);
impl WebViewControlDeferredPermissionRequest {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn PermissionType(&self) -> ::windows_core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlPermissionType>::zeroed();
            (::windows_core::Interface::vtable(this).PermissionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlPermissionType>(result__)
        }
    }
    pub fn Allow(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Allow)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Deny)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlDeferredPermissionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlDeferredPermissionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlDeferredPermissionRequest {}
impl ::core::fmt::Debug for WebViewControlDeferredPermissionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlDeferredPermissionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlDeferredPermissionRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlDeferredPermissionRequest;{2ce349e0-d759-445c-9926-8995298f152b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlDeferredPermissionRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlDeferredPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDeferredPermissionRequest";
}
impl ::core::convert::From<WebViewControlDeferredPermissionRequest> for ::windows_core::IUnknown {
    fn from(value: WebViewControlDeferredPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDeferredPermissionRequest> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlDeferredPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlDeferredPermissionRequest> for ::windows_core::IInspectable {
    fn from(value: WebViewControlDeferredPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDeferredPermissionRequest> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlDeferredPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(::windows_core::IUnknown);
impl WebViewControlLongRunningScriptDetectedEventArgs {
    pub fn ExecutionTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn StopPageScriptExecution(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StopPageScriptExecution)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetStopPageScriptExecution(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStopPageScriptExecution)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlLongRunningScriptDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlLongRunningScriptDetectedEventArgs {}
impl ::core::fmt::Debug for WebViewControlLongRunningScriptDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlLongRunningScriptDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlLongRunningScriptDetectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs;{2a6e5bba-98b4-45bc-bbeb-0f69ce49c599})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlLongRunningScriptDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlLongRunningScriptDetectedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
}
impl ::core::convert::From<WebViewControlLongRunningScriptDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlLongRunningScriptDetectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlLongRunningScriptDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlLongRunningScriptDetectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlNavigationCompletedEventArgs(::windows_core::IUnknown);
impl WebViewControlNavigationCompletedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn IsSuccess(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSuccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn WebErrorStatus(&self) -> ::windows_core::Result<super::WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WebErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).WebErrorStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WebErrorStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNavigationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNavigationCompletedEventArgs {}
impl ::core::fmt::Debug for WebViewControlNavigationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNavigationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlNavigationCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNavigationCompletedEventArgs;{20409918-4a15-4c46-a55d-f79edb0bde8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlNavigationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
}
impl ::core::convert::From<WebViewControlNavigationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNavigationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlNavigationStartingEventArgs(::windows_core::IUnknown);
impl WebViewControlNavigationStartingEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
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
}
impl ::core::clone::Clone for WebViewControlNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNavigationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNavigationStartingEventArgs {}
impl ::core::fmt::Debug for WebViewControlNavigationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNavigationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlNavigationStartingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNavigationStartingEventArgs;{0c9057c5-0a08-41c7-863b-71e3a9549137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlNavigationStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlNavigationStartingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
}
impl ::core::convert::From<WebViewControlNavigationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlNavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationStartingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlNavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNavigationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlNavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationStartingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlNavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlNewWindowRequestedEventArgs(::windows_core::IUnknown);
impl WebViewControlNewWindowRequestedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn Referrer(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Referrer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
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
    pub fn NewWindow(&self) -> ::windows_core::Result<IWebViewControl> {
        let this = &::windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NewWindow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IWebViewControl>(result__)
        }
    }
    pub fn SetNewWindow<'a, Param0: ::windows_core::IntoParam<'a, IWebViewControl>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNewWindow)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = &::windows_core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNewWindowRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNewWindowRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlNewWindowRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNewWindowRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlNewWindowRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs;{3df44bbb-a124-46d5-a083-d02cacdff5ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlNewWindowRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlNewWindowRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlNewWindowRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlNewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNewWindowRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlNewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNewWindowRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlNewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNewWindowRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlNewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlPermissionRequest(::windows_core::IUnknown);
impl WebViewControlPermissionRequest {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn PermissionType(&self) -> ::windows_core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlPermissionType>::zeroed();
            (::windows_core::Interface::vtable(this).PermissionType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlPermissionType>(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<WebViewControlPermissionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlPermissionState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlPermissionState>(result__)
        }
    }
    pub fn Defer(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Defer)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Allow(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Allow)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Deny)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlPermissionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlPermissionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlPermissionRequest {}
impl ::core::fmt::Debug for WebViewControlPermissionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlPermissionRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlPermissionRequest;{e5bc836c-f22f-40e2-95b2-7729f840eb7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlPermissionRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequest";
}
impl ::core::convert::From<WebViewControlPermissionRequest> for ::windows_core::IUnknown {
    fn from(value: WebViewControlPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequest> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlPermissionRequest> for ::windows_core::IInspectable {
    fn from(value: WebViewControlPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequest> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlPermissionRequestedEventArgs(::windows_core::IUnknown);
impl WebViewControlPermissionRequestedEventArgs {
    pub fn PermissionRequest(&self) -> ::windows_core::Result<WebViewControlPermissionRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequest)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlPermissionRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlPermissionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlPermissionRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlPermissionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlPermissionRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlPermissionRequestedEventArgs;{27204d51-2488-4cc5-968e-0a771e59c147})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlPermissionRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlPermissionRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlPermissionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlPermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlPermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlPermissionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlPermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlPermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebViewControlPermissionState(pub i32);
impl WebViewControlPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewControlPermissionState {}
impl ::core::clone::Clone for WebViewControlPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlPermissionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WebViewControlPermissionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlPermissionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlPermissionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebViewControlPermissionType(pub i32);
impl WebViewControlPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewControlPermissionType {}
impl ::core::clone::Clone for WebViewControlPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WebViewControlPermissionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlPermissionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WebViewControlScriptNotifyEventArgs(::windows_core::IUnknown);
impl WebViewControlScriptNotifyEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlScriptNotifyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlScriptNotifyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlScriptNotifyEventArgs {}
impl ::core::fmt::Debug for WebViewControlScriptNotifyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlScriptNotifyEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlScriptNotifyEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlScriptNotifyEventArgs;{491de57b-6f49-41bb-b591-51b85b817037})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlScriptNotifyEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlScriptNotifyEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
}
impl ::core::convert::From<WebViewControlScriptNotifyEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlScriptNotifyEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlScriptNotifyEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlScriptNotifyEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlScriptNotifyEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlScriptNotifyEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlScriptNotifyEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlScriptNotifyEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlSettings(::windows_core::IUnknown);
impl WebViewControlSettings {
    pub fn SetIsJavaScriptEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsJavaScriptEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsJavaScriptEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsJavaScriptEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsIndexedDBEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsIndexedDBEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIndexedDBEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsIndexedDBEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsScriptNotifyAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsScriptNotifyAllowed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsScriptNotifyAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScriptNotifyAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlSettings {}
impl ::core::fmt::Debug for WebViewControlSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlSettings;{c9967fbf-5e98-4cfd-8cce-27b0911e3de8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlSettings {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlSettings";
}
impl ::core::convert::From<WebViewControlSettings> for ::windows_core::IUnknown {
    fn from(value: WebViewControlSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlSettings> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlSettings> for ::windows_core::IInspectable {
    fn from(value: WebViewControlSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlSettings> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(::windows_core::IUnknown);
impl WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
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
impl ::core::clone::Clone for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::fmt::Debug for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlUnsupportedUriSchemeIdentifiedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs;{e3b81944-e4fc-43dc-94ca-f980f30bc51d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
}
impl ::core::convert::From<WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(::windows_core::IUnknown);
impl WebViewControlUnviewableContentIdentifiedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn Referrer(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Referrer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn MediaType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MediaType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlUnviewableContentIdentifiedEventArgs {}
impl ::core::fmt::Debug for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlUnviewableContentIdentifiedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlUnviewableContentIdentifiedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs;{4a9680db-88f2-4e20-b693-b4e2df4aa581})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlUnviewableContentIdentifiedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlUnviewableContentIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
}
impl ::core::convert::From<WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebViewControlWebResourceRequestedEventArgs(::windows_core::IUnknown);
impl WebViewControlWebResourceRequestedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Request(&self) -> ::windows_core::Result<super::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Http::HttpRequestMessage>(result__)
        }
    }
    #[cfg(feature = "winrt-web")]
    pub fn SetResponse<'a, Param0: ::windows_core::IntoParam<'a, super::Http::HttpResponseMessage>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResponse)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-web")]
    pub fn Response(&self) -> ::windows_core::Result<super::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Http::HttpResponseMessage>(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlWebResourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlWebResourceRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlWebResourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlWebResourceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebViewControlWebResourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs;{44d6524d-55a4-4d8b-891c-931d8e25d42e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebViewControlWebResourceRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlWebResourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlWebResourceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebViewControlWebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlWebResourceRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebViewControlWebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlWebResourceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebViewControlWebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlWebResourceRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebViewControlWebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
