#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl ::core::marker::Copy for GazeDeviceConfigurationStatePreview {}
impl ::core::clone::Clone for GazeDeviceConfigurationStatePreview {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GazeDeviceConfigurationStatePreview {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GazeDeviceConfigurationStatePreview {
    type Abi = Self;
}
impl ::core::fmt::Debug for GazeDeviceConfigurationStatePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceConfigurationStatePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GazeDevicePreview(::windows_core::IUnknown);
impl GazeDevicePreview {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CanTrackEyes(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanTrackEyes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanTrackHead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanTrackHead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ConfigurationState(&self) -> ::windows_core::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GazeDeviceConfigurationStatePreview>::zeroed();
            (::windows_core::Interface::vtable(this).ConfigurationState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDeviceConfigurationStatePreview>(result__)
        }
    }
    pub fn RequestCalibrationAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestCalibrationAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlDescriptions)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlDescriptions)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeDevicePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeDevicePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDevicePreview {}
impl ::core::fmt::Debug for GazeDevicePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDevicePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDevicePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDevicePreview;{e79e7ee9-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
    const IID: ::windows_core::GUID = <IGazeDevicePreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
impl ::core::convert::From<GazeDevicePreview> for ::windows_core::IUnknown {
    fn from(value: GazeDevicePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDevicePreview> for ::windows_core::IUnknown {
    fn from(value: &GazeDevicePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeDevicePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeDevicePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeDevicePreview> for ::windows_core::IInspectable {
    fn from(value: GazeDevicePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDevicePreview> for ::windows_core::IInspectable {
    fn from(value: &GazeDevicePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeDevicePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeDevicePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeDevicePreview {}
unsafe impl ::core::marker::Sync for GazeDevicePreview {}
#[repr(transparent)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeDeviceWatcherAddedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherAddedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherAddedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherAddedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherAddedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs;{e79e7eed-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherAddedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[repr(transparent)]
pub struct GazeDeviceWatcherPreview(::windows_core::IUnknown);
impl GazeDeviceWatcherPreview {
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUpdated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for GazeDeviceWatcherPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherPreview {}
impl ::core::fmt::Debug for GazeDeviceWatcherPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherPreview;{e79e7ee7-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
impl ::core::convert::From<GazeDeviceWatcherPreview> for ::windows_core::IUnknown {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherPreview> for ::windows_core::IUnknown {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeDeviceWatcherPreview> for ::windows_core::IInspectable {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherPreview> for ::windows_core::IInspectable {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherPreview {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherPreview {}
#[repr(transparent)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherRemovedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherRemovedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs;{f2631f08-0e3f-431f-a606-50b35af94a1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherRemovedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[repr(transparent)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherUpdatedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherUpdatedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs;{7fe830ef-7f08-4737-88e1-4a83ae4e4885})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherUpdatedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[repr(transparent)]
pub struct GazeEnteredPreviewEventArgs(::windows_core::IUnknown);
impl GazeEnteredPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazePointPreview>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeEnteredPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeEnteredPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeEnteredPreviewEventArgs {}
impl ::core::fmt::Debug for GazeEnteredPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeEnteredPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs;{2567bf43-1225-489f-9dd1-daa7c50fbf4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeEnteredPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
impl ::core::convert::From<GazeEnteredPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeEnteredPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeEnteredPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeEnteredPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeEnteredPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeEnteredPreviewEventArgs {}
#[repr(transparent)]
pub struct GazeExitedPreviewEventArgs(::windows_core::IUnknown);
impl GazeExitedPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazePointPreview>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeExitedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeExitedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeExitedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeExitedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeExitedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs;{5d0af07e-7d83-40ef-9f0a-fbc1bbdcc5ac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeExitedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
impl ::core::convert::From<GazeExitedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeExitedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeExitedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeExitedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeExitedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeExitedPreviewEventArgs {}
#[repr(transparent)]
pub struct GazeInputSourcePreview(::windows_core::IUnknown);
impl GazeInputSourcePreview {
    pub fn GazeMoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GazeMoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGazeMoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeMoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GazeEntered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GazeEntered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGazeEntered<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeEntered)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GazeExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GazeExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGazeExited<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeExited)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeInputSourcePreview>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDeviceWatcherPreview>(result__)
        })
    }
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GazeInputSourcePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeInputSourcePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeInputSourcePreview {}
impl ::core::fmt::Debug for GazeInputSourcePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeInputSourcePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeInputSourcePreview;{e79e7ee8-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
    const IID: ::windows_core::GUID = <IGazeInputSourcePreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
impl ::core::convert::From<GazeInputSourcePreview> for ::windows_core::IUnknown {
    fn from(value: GazeInputSourcePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeInputSourcePreview> for ::windows_core::IUnknown {
    fn from(value: &GazeInputSourcePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeInputSourcePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeInputSourcePreview> for ::windows_core::IInspectable {
    fn from(value: GazeInputSourcePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeInputSourcePreview> for ::windows_core::IInspectable {
    fn from(value: &GazeInputSourcePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeInputSourcePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeInputSourcePreview {}
unsafe impl ::core::marker::Sync for GazeInputSourcePreview {}
#[repr(transparent)]
pub struct GazeMovedPreviewEventArgs(::windows_core::IUnknown);
impl GazeMovedPreviewEventArgs {
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
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazePointPreview>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<GazePointPreview>>(result__)
        }
    }
}
impl ::core::clone::Clone for GazeMovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazeMovedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeMovedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeMovedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeMovedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs;{e79e7eeb-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGazeMovedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
impl ::core::convert::From<GazeMovedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeMovedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazeMovedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazeMovedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazeMovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeMovedPreviewEventArgs {}
#[repr(transparent)]
pub struct GazePointPreview(::windows_core::IUnknown);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SourceDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GazeDevicePreview>(result__)
        }
    }
    pub fn EyeGazePosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EyeGazePosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn HeadGazePosition(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeadGazePosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> ::windows_core::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HidInputReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::HumanInterfaceDevice::HidInputReport>(result__)
        }
    }
}
impl ::core::clone::Clone for GazePointPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GazePointPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazePointPreview {}
impl ::core::fmt::Debug for GazePointPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazePointPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GazePointPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazePointPreview;{e79e7eea-b389-11e7-b201-c8d3ffb75721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
    const IID: ::windows_core::GUID = <IGazePointPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
impl ::core::convert::From<GazePointPreview> for ::windows_core::IUnknown {
    fn from(value: GazePointPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazePointPreview> for ::windows_core::IUnknown {
    fn from(value: &GazePointPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GazePointPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GazePointPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GazePointPreview> for ::windows_core::IInspectable {
    fn from(value: GazePointPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GazePointPreview> for ::windows_core::IInspectable {
    fn from(value: &GazePointPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GazePointPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GazePointPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GazePointPreview {}
unsafe impl ::core::marker::Sync for GazePointPreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDevicePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDevicePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CanTrackEyes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanTrackHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ConfigurationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows_core::HRESULT,
    pub RequestCalibrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetNumericControlDescriptions: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetBooleanControlDescriptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2631f08_0e3f_431f_a606_50b35af94a1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeEnteredPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeEnteredPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeExitedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeInputSourcePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeInputSourcePreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeInputSourcePreviewStatics {
    type Vtable = IGazeInputSourcePreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee6_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeMovedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazePointPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazePointPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SourceDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EyeGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HeadGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub HidInputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))]
    HidInputReport: usize,
}
