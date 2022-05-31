#[repr(transparent)]
pub struct DialApp(::windows_core::IUnknown);
impl DialApp {
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RequestLaunchAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appargument: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialAppLaunchResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestLaunchAsync)(::windows_core::Interface::as_raw(this), appargument.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialAppLaunchResult>>(result__)
        }
    }
    pub fn StopAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialAppStopResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialAppStopResult>>(result__)
        }
    }
    pub fn GetAppStateAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialAppStateDetails>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAppStateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialAppStateDetails>>(result__)
        }
    }
}
impl ::core::clone::Clone for DialApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialApp {}
impl ::core::fmt::Debug for DialApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialApp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialApp {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialApp;{555ffbd3-45b7-49f3-bbd7-302db6084646})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialApp {
    type Vtable = IDialApp_Vtbl;
    const IID: ::windows_core::GUID = <IDialApp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialApp";
}
impl ::core::convert::From<DialApp> for ::windows_core::IUnknown {
    fn from(value: DialApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialApp> for ::windows_core::IUnknown {
    fn from(value: &DialApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialApp> for ::windows_core::IInspectable {
    fn from(value: DialApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialApp> for ::windows_core::IInspectable {
    fn from(value: &DialApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialApp {}
unsafe impl ::core::marker::Sync for DialApp {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: Self = Self(0i32);
    pub const FailedToLaunch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppLaunchResult {}
impl ::core::clone::Clone for DialAppLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DialAppLaunchResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DialAppLaunchResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for DialAppLaunchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialAppLaunchResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialAppLaunchResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppLaunchResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppState {}
impl ::core::clone::Clone for DialAppState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DialAppState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DialAppState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DialAppState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialAppState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialAppState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DialAppStateDetails(::windows_core::IUnknown);
impl DialAppStateDetails {
    pub fn State(&self) -> ::windows_core::Result<DialAppState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DialAppState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DialAppState>(result__)
        }
    }
    pub fn FullXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FullXml)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DialAppStateDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialAppStateDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialAppStateDetails {}
impl ::core::fmt::Debug for DialAppStateDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialAppStateDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialAppStateDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialAppStateDetails;{ddc4a4a1-f5de-400d-bea4-8c8466bb2961})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialAppStateDetails {
    type Vtable = IDialAppStateDetails_Vtbl;
    const IID: ::windows_core::GUID = <IDialAppStateDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialAppStateDetails {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialAppStateDetails";
}
impl ::core::convert::From<DialAppStateDetails> for ::windows_core::IUnknown {
    fn from(value: DialAppStateDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialAppStateDetails> for ::windows_core::IUnknown {
    fn from(value: &DialAppStateDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialAppStateDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialAppStateDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialAppStateDetails> for ::windows_core::IInspectable {
    fn from(value: DialAppStateDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialAppStateDetails> for ::windows_core::IInspectable {
    fn from(value: &DialAppStateDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialAppStateDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialAppStateDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialAppStateDetails {}
unsafe impl ::core::marker::Sync for DialAppStateDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: Self = Self(0i32);
    pub const StopFailed: Self = Self(1i32);
    pub const OperationNotSupported: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppStopResult {}
impl ::core::clone::Clone for DialAppStopResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DialAppStopResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DialAppStopResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for DialAppStopResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialAppStopResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialAppStopResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppStopResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DialDevice(::windows_core::IUnknown);
impl DialDevice {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetDialApp<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appname: Param0) -> ::windows_core::Result<DialApp> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDialApp)(::windows_core::Interface::as_raw(this), appname.into_param().abi(), result__.as_mut_ptr()).from_abi::<DialApp>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows_core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn GetDeviceSelector<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(appname: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), appname.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialDevice>> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialDevice>>(result__)
        })
    }
    #[cfg(feature = "winrt-devices")]
    pub fn DeviceInfoSupportsDialAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_devices::Enumeration::DeviceInformation>>(device: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInfoSupportsDialAsync)(::windows_core::Interface::as_raw(this), device.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IDialDeviceStatics<R, F: FnOnce(&IDialDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DialDevice, IDialDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DialDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialDevice {}
impl ::core::fmt::Debug for DialDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevice;{fff0edaf-759f-41d2-a20a-7f29ce0b3784})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialDevice {
    type Vtable = IDialDevice_Vtbl;
    const IID: ::windows_core::GUID = <IDialDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialDevice {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevice";
}
impl ::core::convert::From<DialDevice> for ::windows_core::IUnknown {
    fn from(value: DialDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevice> for ::windows_core::IUnknown {
    fn from(value: &DialDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialDevice> for ::windows_core::IInspectable {
    fn from(value: DialDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevice> for ::windows_core::IInspectable {
    fn from(value: &DialDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialDevice {}
unsafe impl ::core::marker::Sync for DialDevice {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Disconnected: Self = Self(4i32);
    pub const Error: Self = Self(5i32);
}
impl ::core::marker::Copy for DialDeviceDisplayStatus {}
impl ::core::clone::Clone for DialDeviceDisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DialDeviceDisplayStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DialDeviceDisplayStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DialDeviceDisplayStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDeviceDisplayStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDeviceDisplayStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialDeviceDisplayStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DialDevicePicker(::windows_core::IUnknown);
impl DialDevicePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DialDevicePicker, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Filter(&self) -> ::windows_core::Result<DialDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Filter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DialDevicePickerFilter>(result__)
        }
    }
    #[cfg(feature = "winrt-devices")]
    pub fn Appearance(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Appearance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DevicePickerAppearance>(result__)
        }
    }
    pub fn DialDeviceSelected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DialDevicePicker, DialDeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DialDeviceSelected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDialDeviceSelected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDialDeviceSelected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DisconnectButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DialDevicePicker, DialDisconnectButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisconnectButtonClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisconnectButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisconnectButtonClicked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DialDevicePickerDismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DialDevicePicker, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DialDevicePickerDismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDialDevicePickerDismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDialDevicePickerDismissed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Show<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), selection.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ShowWithPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowWithPlacement)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), preferredplacement).ok() }
    }
    pub fn PickSingleDialDeviceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDialDeviceAsync)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialDevice>>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn PickSingleDialDeviceAsyncWithPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDialDeviceAsyncWithPlacement)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), preferredplacement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DialDevice>>(result__)
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetDisplayStatus<'a, Param0: ::windows_core::IntoParam<'a, DialDevice>>(&self, device: Param0, status: DialDeviceDisplayStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayStatus)(::windows_core::Interface::as_raw(this), device.into_param().abi(), status).ok() }
    }
}
impl ::core::clone::Clone for DialDevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialDevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialDevicePicker {}
impl ::core::fmt::Debug for DialDevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDevicePicker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDevicePicker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevicePicker;{ba7e520a-ff59-4f4b-bdac-d89f495ad6e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialDevicePicker {
    type Vtable = IDialDevicePicker_Vtbl;
    const IID: ::windows_core::GUID = <IDialDevicePicker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialDevicePicker {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePicker";
}
impl ::core::convert::From<DialDevicePicker> for ::windows_core::IUnknown {
    fn from(value: DialDevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevicePicker> for ::windows_core::IUnknown {
    fn from(value: &DialDevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialDevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialDevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialDevicePicker> for ::windows_core::IInspectable {
    fn from(value: DialDevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevicePicker> for ::windows_core::IInspectable {
    fn from(value: &DialDevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialDevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialDevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialDevicePicker {}
unsafe impl ::core::marker::Sync for DialDevicePicker {}
#[repr(transparent)]
pub struct DialDevicePickerFilter(::windows_core::IUnknown);
impl DialDevicePickerFilter {
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedAppNames(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedAppNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for DialDevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialDevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialDevicePickerFilter {}
impl ::core::fmt::Debug for DialDevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDevicePickerFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDevicePickerFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevicePickerFilter;{c17c93ba-86c0-485d-b8d6-0f9a8f641590})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialDevicePickerFilter {
    type Vtable = IDialDevicePickerFilter_Vtbl;
    const IID: ::windows_core::GUID = <IDialDevicePickerFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePickerFilter";
}
impl ::core::convert::From<DialDevicePickerFilter> for ::windows_core::IUnknown {
    fn from(value: DialDevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevicePickerFilter> for ::windows_core::IUnknown {
    fn from(value: &DialDevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialDevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialDevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialDevicePickerFilter> for ::windows_core::IInspectable {
    fn from(value: DialDevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDevicePickerFilter> for ::windows_core::IInspectable {
    fn from(value: &DialDevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialDevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialDevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialDevicePickerFilter {}
unsafe impl ::core::marker::Sync for DialDevicePickerFilter {}
#[repr(transparent)]
pub struct DialDeviceSelectedEventArgs(::windows_core::IUnknown);
impl DialDeviceSelectedEventArgs {
    pub fn SelectedDialDevice(&self) -> ::windows_core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDialDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DialDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for DialDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialDeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialDeviceSelectedEventArgs {}
impl ::core::fmt::Debug for DialDeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDeviceSelectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDeviceSelectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDeviceSelectedEventArgs;{480b92ad-ac76-47eb-9c06-a19304da0247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialDeviceSelectedEventArgs {
    type Vtable = IDialDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDialDeviceSelectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDeviceSelectedEventArgs";
}
impl ::core::convert::From<DialDeviceSelectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DialDeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDeviceSelectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DialDeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialDeviceSelectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DialDeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDeviceSelectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DialDeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialDeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DialDeviceSelectedEventArgs {}
#[repr(transparent)]
pub struct DialDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
impl DialDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DialDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for DialDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialDisconnectButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialDisconnectButtonClickedEventArgs {}
impl ::core::fmt::Debug for DialDisconnectButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialDisconnectButtonClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs;{52765152-9c81-4e55-adc2-0ebe99cde3b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialDisconnectButtonClickedEventArgs {
    type Vtable = IDialDisconnectButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDialDisconnectButtonClickedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs";
}
impl ::core::convert::From<DialDisconnectButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DialDisconnectButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDisconnectButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DialDisconnectButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialDisconnectButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DialDisconnectButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialDisconnectButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DialDisconnectButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DialDisconnectButtonClickedEventArgs {}
#[repr(transparent)]
pub struct DialReceiverApp(::windows_core::IUnknown);
impl DialReceiverApp {
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAdditionalDataAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAdditionalDataAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAdditionalDataAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(&self, additionaldata: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetAdditionalDataAsync)(::windows_core::Interface::as_raw(this), additionaldata.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetUniqueDeviceNameAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IDialReceiverApp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUniqueDeviceNameAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<DialReceiverApp> {
        Self::IDialReceiverAppStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DialReceiverApp>(result__)
        })
    }
    pub fn IDialReceiverAppStatics<R, F: FnOnce(&IDialReceiverAppStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DialReceiverApp, IDialReceiverAppStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DialReceiverApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialReceiverApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialReceiverApp {}
impl ::core::fmt::Debug for DialReceiverApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialReceiverApp").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialReceiverApp {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialReceiverApp;{fd3e7c57-5045-470e-b304-4dd9b13e7d11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialReceiverApp {
    type Vtable = IDialReceiverApp_Vtbl;
    const IID: ::windows_core::GUID = <IDialReceiverApp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialReceiverApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialReceiverApp";
}
impl ::core::convert::From<DialReceiverApp> for ::windows_core::IUnknown {
    fn from(value: DialReceiverApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverApp> for ::windows_core::IUnknown {
    fn from(value: &DialReceiverApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialReceiverApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialReceiverApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialReceiverApp> for ::windows_core::IInspectable {
    fn from(value: DialReceiverApp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverApp> for ::windows_core::IInspectable {
    fn from(value: &DialReceiverApp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialReceiverApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialReceiverApp {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DialReceiverApp {}
unsafe impl ::core::marker::Sync for DialReceiverApp {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialApp {
    type Vtable = IDialApp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x555ffbd3_45b7_49f3_bbd7_302db6084646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialApp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RequestLaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appargument: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAppStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialAppStateDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialAppStateDetails {
    type Vtable = IDialAppStateDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddc4a4a1_f5de_400d_bea4_8c8466bb2961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialAppStateDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DialAppState) -> ::windows_core::HRESULT,
    pub FullXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDevice {
    type Vtable = IDialDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfff0edaf_759f_41d2_a20a_7f29ce0b3784);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDialApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDevice2 {
    type Vtable = IDialDevice2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbab7f3d5_5bfb_4eba_8b32_b57c5c5ee5c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevice2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDevicePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDevicePicker {
    type Vtable = IDialDevicePicker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xba7e520a_ff59_4f4b_bdac_d89f495ad6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevicePicker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    Appearance: usize,
    pub DialDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDialDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DialDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDialDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ShowWithPlacement: usize,
    pub PickSingleDialDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub PickSingleDialDeviceAsyncWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, preferredplacement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    PickSingleDialDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows_core::RawPtr, status: DialDeviceDisplayStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDevicePickerFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDevicePickerFilter {
    type Vtable = IDialDevicePickerFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc17c93ba_86c0_485d_b8d6_0f9a8f641590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevicePickerFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedAppNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedAppNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDeviceSelectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDeviceSelectedEventArgs {
    type Vtable = IDialDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x480b92ad_ac76_47eb_9c06_a19304da0247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDeviceSelectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedDialDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDeviceStatics {
    type Vtable = IDialDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa69cc95_01f8_4758_8461_2bbd1cdc3cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-devices")]
    pub DeviceInfoSupportsDialAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-devices"))]
    DeviceInfoSupportsDialAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialDisconnectButtonClickedEventArgs {
    type Vtable = IDialDisconnectButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52765152_9c81_4e55_adc2_0ebe99cde3b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDisconnectButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialReceiverApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialReceiverApp {
    type Vtable = IDialReceiverApp_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd3e7c57_5045_470e_b304_4dd9b13e7d11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverApp_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetAdditionalDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetAdditionalDataAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetAdditionalDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, additionaldata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetAdditionalDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialReceiverApp2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialReceiverApp2 {
    type Vtable = IDialReceiverApp2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x530c5805_9130_42ac_a504_1977dcb2ea8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverApp2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetUniqueDeviceNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDialReceiverAppStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDialReceiverAppStatics {
    type Vtable = IDialReceiverAppStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53183a3c_4c36_4d02_b28a_f2a9da38ec52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverAppStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
