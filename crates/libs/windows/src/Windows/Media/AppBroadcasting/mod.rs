#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingMonitor(::windows_core::IUnknown);
impl AppBroadcastingMonitor {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppBroadcastingMonitor, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsCurrentAppBroadcasting(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAppBroadcasting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppBroadcastingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAppBroadcastingChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsCurrentAppBroadcastingChanged<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsCurrentAppBroadcastingChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastingMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingMonitor {}
impl ::core::fmt::Debug for AppBroadcastingMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastingMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingMonitor;{00f95a68-8907-48a0-b8ef-24d208137542})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastingMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
}
impl ::core::convert::From<AppBroadcastingMonitor> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastingMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingMonitor> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastingMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastingMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastingMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingMonitor> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastingMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingMonitor> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastingMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastingMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastingMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingMonitor {}
unsafe impl ::core::marker::Sync for AppBroadcastingMonitor {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatus(::windows_core::IUnknown);
impl AppBroadcastingStatus {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn CanStartBroadcast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanStartBroadcast)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn Details(&self) -> ::windows_core::Result<AppBroadcastingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastingStatusDetails>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingStatus {}
impl ::core::fmt::Debug for AppBroadcastingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastingStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatus;{1225e4df-03a1-42f8-8b80-c9228cd9cf2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastingStatus as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatus";
}
impl ::core::convert::From<AppBroadcastingStatus> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatus> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingStatus> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatus> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastingStatus {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingStatus {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatus {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatusDetails(::windows_core::IUnknown);
impl AppBroadcastingStatusDetails {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsAnyAppBroadcasting(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnyAppBroadcasting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptureResourceUnavailable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsGameStreamInProgress(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGameStreamInProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsAppInactive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppInactive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsBlockedForApp(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockedForApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsDisabledByUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsDisabledBySystem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledBySystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingStatusDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingStatusDetails {}
impl ::core::fmt::Debug for AppBroadcastingStatusDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingStatusDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastingStatusDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails;{069dada4-b573-4e3c-8e19-1bafacd09713})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastingStatusDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
}
impl ::core::convert::From<AppBroadcastingStatusDetails> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatusDetails> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingStatusDetails> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatusDetails> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingStatusDetails {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatusDetails {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingUI(::windows_core::IUnknown);
impl AppBroadcastingUI {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn GetStatus(&self) -> ::windows_core::Result<AppBroadcastingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn ShowBroadcastUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowBroadcastUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn GetDefault() -> ::windows_core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows_core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastingUIStatics<R, F: FnOnce(&IAppBroadcastingUIStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppBroadcastingUI, IAppBroadcastingUIStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppBroadcastingUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingUI {}
impl ::core::fmt::Debug for AppBroadcastingUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBroadcastingUI {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingUI;{e56f9f8f-ee99-4dca-a3c3-70af3db44f5f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
    const IID: ::windows_core::GUID = <IAppBroadcastingUI as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingUI";
}
impl ::core::convert::From<AppBroadcastingUI> for ::windows_core::IUnknown {
    fn from(value: AppBroadcastingUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingUI> for ::windows_core::IUnknown {
    fn from(value: &AppBroadcastingUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBroadcastingUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBroadcastingUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingUI> for ::windows_core::IInspectable {
    fn from(value: AppBroadcastingUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingUI> for ::windows_core::IInspectable {
    fn from(value: &AppBroadcastingUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBroadcastingUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBroadcastingUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingUI {}
unsafe impl ::core::marker::Sync for AppBroadcastingUI {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f95a68_8907_48a0_b8ef_24d208137542);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCurrentAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCurrentAppBroadcastingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsCurrentAppBroadcastingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1225e4df_03a1_42f8_8b80_c9228cd9cf2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatus_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanStartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingStatusDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x069dada4_b573_4e3c_8e19_1bafacd09713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatusDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe56f9f8f_ee99_4dca_a3c3_70af3db44f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUI_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingUIStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastingUIStatics {
    type Vtable = IAppBroadcastingUIStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55a8a79d_23cb_4579_9c34_886fe02c045a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUIStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
