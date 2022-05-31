#[cfg(feature = "Pnp")]
pub mod Pnp;
#[repr(transparent)]
pub struct DeviceAccessChangedEventArgs(::windows_core::IUnknown);
impl DeviceAccessChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccessStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IDeviceAccessChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceAccessChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceAccessChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessChangedEventArgs {}
impl ::core::fmt::Debug for DeviceAccessChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccessChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessChangedEventArgs;{deda0bcc-4f9d-4f58-9dba-a9bc800408d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceAccessChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceAccessChangedEventArgs {}
#[repr(transparent)]
pub struct DeviceAccessInformation(::windows_core::IUnknown);
impl DeviceAccessInformation {
    pub fn AccessChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AccessChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccessChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn CurrentStatus(&self) -> ::windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceAccessStatus>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    pub fn CreateFromId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromId)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    pub fn CreateFromDeviceClassId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(deviceclassid: Param0) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceClassId)(::windows_core::Interface::as_raw(this), deviceclassid.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    pub fn CreateFromDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, result__.as_mut_ptr()).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    pub fn IDeviceAccessInformationStatics<R, F: FnOnce(&IDeviceAccessInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceAccessInformation, IDeviceAccessInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeviceAccessInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceAccessInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessInformation {}
impl ::core::fmt::Debug for DeviceAccessInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccessInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessInformation;{0baa9a73-6de5-4915-8ddd-9a0554a6f545})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceAccessInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessInformation";
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows_core::IUnknown {
    fn from(value: DeviceAccessInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows_core::IUnknown {
    fn from(value: &DeviceAccessInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceAccessInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows_core::IInspectable {
    fn from(value: DeviceAccessInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows_core::IInspectable {
    fn from(value: &DeviceAccessInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceAccessInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceAccessInformation {}
unsafe impl ::core::marker::Sync for DeviceAccessInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccessStatus {}
impl ::core::clone::Clone for DeviceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: Self = Self(0i32);
    pub const AudioCapture: Self = Self(1i32);
    pub const AudioRender: Self = Self(2i32);
    pub const PortableStorageDevice: Self = Self(3i32);
    pub const VideoCapture: Self = Self(4i32);
    pub const ImageScanner: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceClass {}
impl ::core::clone::Clone for DeviceClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeviceConnectionChangeTriggerDetails(::windows_core::IUnknown);
impl DeviceConnectionChangeTriggerDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceConnectionChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceConnectionChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceConnectionChangeTriggerDetails {}
impl ::core::fmt::Debug for DeviceConnectionChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceConnectionChangeTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceConnectionChangeTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails;{b8578c0c-bbc1-484b-bffa-7b31dcc200b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceConnectionChangeTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTriggerDetails {}
#[repr(transparent)]
pub struct DeviceDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
impl DeviceDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceDisconnectButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceDisconnectButtonClickedEventArgs {}
impl ::core::fmt::Debug for DeviceDisconnectButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDisconnectButtonClickedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs;{8e44b56d-f902-4a00-b536-f37992e6a2a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceDisconnectButtonClickedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceDisconnectButtonClickedEventArgs {}
#[repr(transparent)]
pub struct DeviceInformation(::windows_core::IUnknown);
impl DeviceInformation {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EnclosureLocation(&self) -> ::windows_core::Result<EnclosureLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnclosureLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EnclosureLocation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn Update<'a, Param0: ::windows_core::IntoParam<'a, DeviceInformationUpdate>>(&self, updateinfo: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), updateinfo.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetGlyphThumbnailAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetGlyphThumbnailAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<DeviceInformationKind> {
        let this = &::windows_core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceInformationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformationKind>(result__)
        }
    }
    pub fn Pairing(&self) -> ::windows_core::Result<DeviceInformationPairing> {
        let this = &::windows_core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Pairing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformationPairing>(result__)
        }
    }
    pub fn CreateFromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(deviceid: Param0, additionalproperties: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsyncAdditionalProperties)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(aqsfilter: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncAqsFilter)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilterAndAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn CreateWatcherDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, result__.as_mut_ptr()).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn CreateWatcherAqsFilter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(aqsfilter: Param0) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherAqsFilter)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherAqsFilterAndAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), result__.as_mut_ptr()).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn GetAqsFilterFromDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAqsFilterFromDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncWithKindAndAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(deviceid: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsyncWithKindAndAdditionalProperties)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), kind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncWithKindAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherWithKindAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, result__.as_mut_ptr()).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn IDeviceInformationStatics<R, F: FnOnce(&IDeviceInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceInformation, IDeviceInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationStatics2<R, F: FnOnce(&IDeviceInformationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceInformation, IDeviceInformationStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformation {}
impl ::core::fmt::Debug for DeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceInformation {
    type Vtable = IDeviceInformation_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformation";
}
impl ::core::convert::From<DeviceInformation> for ::windows_core::IUnknown {
    fn from(value: DeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows_core::IUnknown {
    fn from(value: &DeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceInformation> for ::windows_core::IInspectable {
    fn from(value: DeviceInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows_core::IInspectable {
    fn from(value: &DeviceInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceInformation {}
unsafe impl ::core::marker::Sync for DeviceInformation {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct DeviceInformationCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl DeviceInformationCollection {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<DeviceInformation>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<DeviceInformation>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<DeviceInformation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, DeviceInformation>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<DeviceInformation>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for DeviceInformationCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for DeviceInformationCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for DeviceInformationCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::RuntimeType for DeviceInformationCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for DeviceInformationCollection {
    type Vtable = ::winrt_foundation::Collections::IVectorView_Vtbl<DeviceInformation>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVectorView<DeviceInformation> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for DeviceInformationCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = ::winrt_foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows_core::IUnknown {
    fn from(value: DeviceInformationCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows_core::IUnknown {
    fn from(value: &DeviceInformationCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows_core::IInspectable {
    fn from(value: DeviceInformationCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows_core::IInspectable {
    fn from(value: &DeviceInformationCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DeviceInformationCollection> for ::winrt_foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceInformationCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DeviceInformationCollection> for ::winrt_foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceInformationCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<DeviceInformation>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<DeviceInformation>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<DeviceInformation>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DeviceInformationCollection> for ::winrt_foundation::Collections::IVectorView<DeviceInformation> {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceInformationCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DeviceInformationCollection> for ::winrt_foundation::Collections::IVectorView<DeviceInformation> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceInformationCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<DeviceInformation>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVectorView<DeviceInformation>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVectorView<DeviceInformation>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DeviceInformationCollection {}
#[repr(transparent)]
pub struct DeviceInformationCustomPairing(::windows_core::IUnknown);
impl DeviceInformationCustomPairing {
    pub fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param2: ::windows_core::IntoParam<'a, IDevicePairingSettings>>(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, devicepairingsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn PairingRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PairingRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePairingRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePairingRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DeviceInformationCustomPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceInformationCustomPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationCustomPairing {}
impl ::core::fmt::Debug for DeviceInformationCustomPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCustomPairing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceInformationCustomPairing {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCustomPairing;{85138c02-4ee6-4914-8370-107a39144c0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceInformationCustomPairing as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCustomPairing";
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows_core::IUnknown {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows_core::IUnknown {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows_core::IInspectable {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows_core::IInspectable {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceInformationCustomPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationCustomPairing {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
impl ::core::marker::Copy for DeviceInformationKind {}
impl ::core::clone::Clone for DeviceInformationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceInformationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceInformationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceInformationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceInformationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceInformationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeviceInformationPairing(::windows_core::IUnknown);
impl DeviceInformationPairing {
    pub fn IsPaired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPaired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanPair(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanPair)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PairAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(::windows_core::Interface::as_raw(this), minprotectionlevel, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn ProtectionLevel(&self) -> ::windows_core::Result<DevicePairingProtectionLevel> {
        let this = &::windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DevicePairingProtectionLevel>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
    pub fn Custom(&self) -> ::windows_core::Result<DeviceInformationCustomPairing> {
        let this = &::windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Custom)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformationCustomPairing>(result__)
        }
    }
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param1: ::windows_core::IntoParam<'a, IDevicePairingSettings>>(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DevicePairingResult>> {
        let this = &::windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(::windows_core::Interface::as_raw(this), minprotectionlevel, devicepairingsettings.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    pub fn UnpairAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceUnpairingResult>> {
        let this = &::windows_core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnpairAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceUnpairingResult>>(result__)
        }
    }
    pub fn TryRegisterForAllInboundPairingRequests(pairingkindssupported: DevicePairingKinds) -> ::windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequests)(::windows_core::Interface::as_raw(this), pairingkindssupported, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequestsWithProtectionLevel)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IDeviceInformationPairingStatics<R, F: FnOnce(&IDeviceInformationPairingStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationPairingStatics2<R, F: FnOnce(&IDeviceInformationPairingStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeviceInformationPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceInformationPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationPairing {}
impl ::core::fmt::Debug for DeviceInformationPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationPairing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceInformationPairing {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationPairing;{2c4769f5-f684-40d5-8469-e8dbaab70485})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceInformationPairing as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationPairing";
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows_core::IUnknown {
    fn from(value: DeviceInformationPairing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows_core::IUnknown {
    fn from(value: &DeviceInformationPairing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceInformationPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows_core::IInspectable {
    fn from(value: DeviceInformationPairing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows_core::IInspectable {
    fn from(value: &DeviceInformationPairing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceInformationPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceInformationPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationPairing {}
#[repr(transparent)]
pub struct DeviceInformationUpdate(::windows_core::IUnknown);
impl DeviceInformationUpdate {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<DeviceInformationKind> {
        let this = &::windows_core::Interface::cast::<IDeviceInformationUpdate2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceInformationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceInformationUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceInformationUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationUpdate {}
impl ::core::fmt::Debug for DeviceInformationUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationUpdate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceInformationUpdate {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationUpdate;{8f315305-d972-44b7-a37e-9e822c78213b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceInformationUpdate as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationUpdate";
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows_core::IUnknown {
    fn from(value: DeviceInformationUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows_core::IUnknown {
    fn from(value: &DeviceInformationUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows_core::IInspectable {
    fn from(value: DeviceInformationUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows_core::IInspectable {
    fn from(value: &DeviceInformationUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceInformationUpdate {}
unsafe impl ::core::marker::Sync for DeviceInformationUpdate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: Self = Self(0u32);
    pub const ConfirmOnly: Self = Self(1u32);
    pub const DisplayPin: Self = Self(2u32);
    pub const ProvidePin: Self = Self(4u32);
    pub const ConfirmPinMatch: Self = Self(8u32);
    pub const ProvidePasswordCredential: Self = Self(16u32);
}
impl ::core::marker::Copy for DevicePairingKinds {}
impl ::core::clone::Clone for DevicePairingKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DevicePairingKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePairingKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DevicePairingKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePairingKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePairingKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePairingKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePairingKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingKinds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Encryption: Self = Self(2i32);
    pub const EncryptionAndAuthentication: Self = Self(3i32);
}
impl ::core::marker::Copy for DevicePairingProtectionLevel {}
impl ::core::clone::Clone for DevicePairingProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DevicePairingProtectionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePairingProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingProtectionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingProtectionLevel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingProtectionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DevicePairingRequestedEventArgs(::windows_core::IUnknown);
impl DevicePairingRequestedEventArgs {
    pub fn DeviceInformation(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformation>(result__)
        }
    }
    pub fn PairingKind(&self) -> ::windows_core::Result<DevicePairingKinds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DevicePairingKinds>::zeroed();
            (::windows_core::Interface::vtable(this).PairingKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePairingKinds>(result__)
        }
    }
    pub fn Pin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptWithPin<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, pin: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptWithPin)(::windows_core::Interface::as_raw(this), pin.into_param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn AcceptWithPasswordCredential<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, passwordcredential: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDevicePairingRequestedEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AcceptWithPasswordCredential)(::windows_core::Interface::as_raw(this), passwordcredential.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DevicePairingRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePairingRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingRequestedEventArgs {}
impl ::core::fmt::Debug for DevicePairingRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingRequestedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingRequestedEventArgs;{f717fc56-de6b-487f-8376-0180aca69963})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePairingRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePairingRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingRequestedEventArgs {}
#[repr(transparent)]
pub struct DevicePairingResult(::windows_core::IUnknown);
impl DevicePairingResult {
    pub fn Status(&self) -> ::windows_core::Result<DevicePairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DevicePairingResultStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePairingResultStatus>(result__)
        }
    }
    pub fn ProtectionLevelUsed(&self) -> ::windows_core::Result<DevicePairingProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DevicePairingProtectionLevel>::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevelUsed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingResult {}
impl ::core::fmt::Debug for DevicePairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingResult;{072b02bf-dd95-4025-9b37-de51adba37b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePairingResult {
    type Vtable = IDevicePairingResult_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePairingResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingResult";
}
impl ::core::convert::From<DevicePairingResult> for ::windows_core::IUnknown {
    fn from(value: DevicePairingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows_core::IUnknown {
    fn from(value: &DevicePairingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePairingResult> for ::windows_core::IInspectable {
    fn from(value: DevicePairingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows_core::IInspectable {
    fn from(value: &DevicePairingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePairingResult {}
unsafe impl ::core::marker::Sync for DevicePairingResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: Self = Self(0i32);
    pub const NotReadyToPair: Self = Self(1i32);
    pub const NotPaired: Self = Self(2i32);
    pub const AlreadyPaired: Self = Self(3i32);
    pub const ConnectionRejected: Self = Self(4i32);
    pub const TooManyConnections: Self = Self(5i32);
    pub const HardwareFailure: Self = Self(6i32);
    pub const AuthenticationTimeout: Self = Self(7i32);
    pub const AuthenticationNotAllowed: Self = Self(8i32);
    pub const AuthenticationFailure: Self = Self(9i32);
    pub const NoSupportedProfiles: Self = Self(10i32);
    pub const ProtectionLevelCouldNotBeMet: Self = Self(11i32);
    pub const AccessDenied: Self = Self(12i32);
    pub const InvalidCeremonyData: Self = Self(13i32);
    pub const PairingCanceled: Self = Self(14i32);
    pub const OperationAlreadyInProgress: Self = Self(15i32);
    pub const RequiredHandlerNotRegistered: Self = Self(16i32);
    pub const RejectedByHandler: Self = Self(17i32);
    pub const RemoteDeviceHasAssociation: Self = Self(18i32);
    pub const Failed: Self = Self(19i32);
}
impl ::core::marker::Copy for DevicePairingResultStatus {}
impl ::core::clone::Clone for DevicePairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DevicePairingResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingResultStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DevicePicker(::windows_core::IUnknown);
impl DevicePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DevicePicker, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Filter(&self) -> ::windows_core::Result<DevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Filter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePickerFilter>(result__)
        }
    }
    pub fn Appearance(&self) -> ::windows_core::Result<DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Appearance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DevicePickerAppearance>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestedProperties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn DeviceSelected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceSelected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDeviceSelected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDeviceSelected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DisconnectButtonClicked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn DevicePickerDismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DevicePicker, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DevicePickerDismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDevicePickerDismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDevicePickerDismissed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Show<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), selection.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowWithPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0, placement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowWithPlacement)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), placement).ok() }
    }
    pub fn PickSingleDeviceAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDeviceAsync)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn PickSingleDeviceAsyncWithPlacement<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, selection: Param0, placement: ::winrt_ui::Popups::Placement) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDeviceAsyncWithPlacement)(::windows_core::Interface::as_raw(this), selection.into_param().abi(), placement, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetDisplayStatus<'a, Param0: ::windows_core::IntoParam<'a, DeviceInformation>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, device: Param0, status: Param1, options: DevicePickerDisplayStatusOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayStatus)(::windows_core::Interface::as_raw(this), device.into_param().abi(), status.into_param().abi(), options).ok() }
    }
}
impl ::core::clone::Clone for DevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePicker {}
impl ::core::fmt::Debug for DevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePicker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePicker {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePicker;{84997aa2-034a-4440-8813-7d0bd479bf5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePicker {
    type Vtable = IDevicePicker_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePicker as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePicker";
}
impl ::core::convert::From<DevicePicker> for ::windows_core::IUnknown {
    fn from(value: DevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows_core::IUnknown {
    fn from(value: &DevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePicker> for ::windows_core::IInspectable {
    fn from(value: DevicePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows_core::IInspectable {
    fn from(value: &DevicePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePicker {}
unsafe impl ::core::marker::Sync for DevicePicker {}
#[repr(transparent)]
pub struct DevicePickerAppearance(::windows_core::IUnknown);
impl DevicePickerAppearance {
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
    #[cfg(feature = "UI")]
    pub fn ForegroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn AccentColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).AccentColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetAccentColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAccentColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedForegroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForegroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedForegroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedForegroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedBackgroundColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SelectedAccentColor(&self) -> ::windows_core::Result<::winrt_ui::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_ui::Color>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedAccentColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetSelectedAccentColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_ui::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedAccentColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DevicePickerAppearance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePickerAppearance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerAppearance {}
impl ::core::fmt::Debug for DevicePickerAppearance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerAppearance").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePickerAppearance {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerAppearance;{e69a12c6-e627-4ed8-9b6c-460af445e56d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePickerAppearance as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerAppearance";
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows_core::IUnknown {
    fn from(value: DevicePickerAppearance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows_core::IUnknown {
    fn from(value: &DevicePickerAppearance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePickerAppearance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows_core::IInspectable {
    fn from(value: DevicePickerAppearance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows_core::IInspectable {
    fn from(value: &DevicePickerAppearance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePickerAppearance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePickerAppearance {}
unsafe impl ::core::marker::Sync for DevicePickerAppearance {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: Self = Self(0u32);
    pub const ShowProgress: Self = Self(1u32);
    pub const ShowDisconnectButton: Self = Self(2u32);
    pub const ShowRetryButton: Self = Self(4u32);
}
impl ::core::marker::Copy for DevicePickerDisplayStatusOptions {}
impl ::core::clone::Clone for DevicePickerDisplayStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePickerDisplayStatusOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DevicePickerDisplayStatusOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DevicePickerDisplayStatusOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerDisplayStatusOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePickerDisplayStatusOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePickerDisplayStatusOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePickerDisplayStatusOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DevicePickerFilter(::windows_core::IUnknown);
impl DevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceClasses(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<DeviceClass>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDeviceClasses)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<DeviceClass>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceSelectors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDeviceSelectors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerFilter {}
impl ::core::fmt::Debug for DevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePickerFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerFilter;{91db92a2-57cb-48f1-9b59-a59b7a1f02a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePickerFilter {
    type Vtable = IDevicePickerFilter_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePickerFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerFilter";
}
impl ::core::convert::From<DevicePickerFilter> for ::windows_core::IUnknown {
    fn from(value: DevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows_core::IUnknown {
    fn from(value: &DevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePickerFilter> for ::windows_core::IInspectable {
    fn from(value: DevicePickerFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows_core::IInspectable {
    fn from(value: &DevicePickerFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DevicePickerFilter {}
unsafe impl ::core::marker::Sync for DevicePickerFilter {}
#[repr(transparent)]
pub struct DeviceSelectedEventArgs(::windows_core::IUnknown);
impl DeviceSelectedEventArgs {
    pub fn SelectedDevice(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDevice)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceSelectedEventArgs {}
impl ::core::fmt::Debug for DeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceSelectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceSelectedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceSelectedEventArgs;{269edade-1d2f-4940-8402-4156b81d3c77})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceSelectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceSelectedEventArgs";
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceSelectedEventArgs {}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct DeviceThumbnail(::windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl DeviceThumbnail {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: ::winrt_storage::Streams::InputStreamOptions) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), count, options, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<::winrt_storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn WriteAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IInputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<::winrt_storage::Streams::IOutputStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows_core::Result<::winrt_storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for DeviceThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for DeviceThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for DeviceThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::RuntimeType for DeviceThumbnail {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::Interface for DeviceThumbnail {
    type Vtable = ::winrt_storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_storage::Streams::IRandomAccessStreamWithContentType as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeName for DeviceThumbnail {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceThumbnail";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows_core::IUnknown {
    fn from(value: DeviceThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows_core::IUnknown {
    fn from(value: &DeviceThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows_core::IInspectable {
    fn from(value: DeviceThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows_core::IInspectable {
    fn from(value: &DeviceThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_storage::Streams::IContentTypeProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IContentTypeProvider> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IContentTypeProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_storage::Streams::IInputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IInputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IInputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_storage::Streams::IOutputStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IOutputStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_storage::Streams::IRandomAccessStream {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStream>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for ::winrt_storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> for &DeviceThumbnail {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<::winrt_storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for DeviceThumbnail {}
#[repr(transparent)]
pub struct DeviceUnpairingResult(::windows_core::IUnknown);
impl DeviceUnpairingResult {
    pub fn Status(&self) -> ::windows_core::Result<DeviceUnpairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceUnpairingResultStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceUnpairingResultStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceUnpairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceUnpairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUnpairingResult {}
impl ::core::fmt::Debug for DeviceUnpairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceUnpairingResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceUnpairingResult;{66f44ad3-79d9-444b-92cf-a92ef72571c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceUnpairingResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceUnpairingResult";
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows_core::IUnknown {
    fn from(value: DeviceUnpairingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows_core::IUnknown {
    fn from(value: &DeviceUnpairingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows_core::IInspectable {
    fn from(value: DeviceUnpairingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows_core::IInspectable {
    fn from(value: &DeviceUnpairingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceUnpairingResult {}
unsafe impl ::core::marker::Sync for DeviceUnpairingResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: Self = Self(0i32);
    pub const AlreadyUnpaired: Self = Self(1i32);
    pub const OperationAlreadyInProgress: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for DeviceUnpairingResultStatus {}
impl ::core::clone::Clone for DeviceUnpairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceUnpairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceUnpairingResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceUnpairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceUnpairingResultStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceUnpairingResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeviceWatcher(::windows_core::IUnknown);
impl DeviceWatcher {
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DeviceWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub fn GetBackgroundTrigger<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DeviceWatcherEventKind>>>(&self, requestedeventkinds: Param0) -> ::windows_core::Result<::winrt_applicationmodel::Background::DeviceWatcherTrigger> {
        let this = &::windows_core::Interface::cast::<IDeviceWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBackgroundTrigger)(::windows_core::Interface::as_raw(this), requestedeventkinds.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Background::DeviceWatcherTrigger>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcher {}
impl ::core::fmt::Debug for DeviceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcher;{c9eab97d-8f6b-4f96-a9f4-abc814e22271})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcher {
    type Vtable = IDeviceWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcher";
}
impl ::core::convert::From<DeviceWatcher> for ::windows_core::IUnknown {
    fn from(value: DeviceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows_core::IUnknown {
    fn from(value: &DeviceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceWatcher> for ::windows_core::IInspectable {
    fn from(value: DeviceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows_core::IInspectable {
    fn from(value: &DeviceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceWatcher {}
unsafe impl ::core::marker::Sync for DeviceWatcher {}
#[repr(transparent)]
pub struct DeviceWatcherEvent(::windows_core::IUnknown);
impl DeviceWatcherEvent {
    pub fn Kind(&self) -> ::windows_core::Result<DeviceWatcherEventKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DeviceWatcherEventKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceWatcherEventKind>(result__)
        }
    }
    pub fn DeviceInformation(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformation>(result__)
        }
    }
    pub fn DeviceInformationUpdate(&self) -> ::windows_core::Result<DeviceInformationUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationUpdate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DeviceInformationUpdate>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceWatcherEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherEvent {}
impl ::core::fmt::Debug for DeviceWatcherEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceWatcherEvent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherEvent;{74aa9c0b-1dbd-47fd-b635-3cc556d0ff8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceWatcherEvent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherEvent";
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows_core::IUnknown {
    fn from(value: DeviceWatcherEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows_core::IUnknown {
    fn from(value: &DeviceWatcherEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows_core::IInspectable {
    fn from(value: DeviceWatcherEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows_core::IInspectable {
    fn from(value: &DeviceWatcherEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceWatcherEvent {}
unsafe impl ::core::marker::Sync for DeviceWatcherEvent {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Remove: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceWatcherEventKind {}
impl ::core::clone::Clone for DeviceWatcherEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceWatcherEventKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceWatcherEventKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceWatcherEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEventKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceWatcherEventKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherEventKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DeviceWatcherStatus {}
impl ::core::clone::Clone for DeviceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeviceWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DeviceWatcherTriggerDetails(::windows_core::IUnknown);
impl DeviceWatcherTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceWatcherEvents(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<DeviceWatcherEvent>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceWatcherEvents)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<DeviceWatcherEvent>>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherTriggerDetails {}
impl ::core::fmt::Debug for DeviceWatcherTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceWatcherTriggerDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherTriggerDetails;{38808119-4cb7-4e57-a56d-776d07cbfef9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceWatcherTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows_core::IUnknown {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows_core::IInspectable {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceWatcherTriggerDetails {}
#[repr(transparent)]
pub struct EnclosureLocation(::windows_core::IUnknown);
impl EnclosureLocation {
    pub fn InDock(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InDock)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn InLid(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InLid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Panel(&self) -> ::windows_core::Result<Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Panel>::zeroed();
            (::windows_core::Interface::vtable(this).Panel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Panel>(result__)
        }
    }
    pub fn RotationAngleInDegreesClockwise(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IEnclosureLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegreesClockwise)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for EnclosureLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnclosureLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnclosureLocation {}
impl ::core::fmt::Debug for EnclosureLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnclosureLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EnclosureLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.EnclosureLocation;{42340a27-5810-459c-aabb-c65e1f813ecf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EnclosureLocation {
    type Vtable = IEnclosureLocation_Vtbl;
    const IID: ::windows_core::GUID = <IEnclosureLocation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.EnclosureLocation";
}
impl ::core::convert::From<EnclosureLocation> for ::windows_core::IUnknown {
    fn from(value: EnclosureLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows_core::IUnknown {
    fn from(value: &EnclosureLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnclosureLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnclosureLocation> for ::windows_core::IInspectable {
    fn from(value: EnclosureLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows_core::IInspectable {
    fn from(value: &EnclosureLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnclosureLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnclosureLocation {}
unsafe impl ::core::marker::Sync for EnclosureLocation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessChangedEventArgs2 {
    type Vtable = IDeviceAccessChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82523262_934b_4b30_a178_adc39f2f2be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessInformationStatics {
    type Vtable = IDeviceAccessInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x574bd3d3_5f30_45cd_8a94_724fe5973084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromDeviceClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclassid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformation {
    type Vtable = IDeviceInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetThumbnailAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetGlyphThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetGlyphThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformation2 {
    type Vtable = IDeviceInformation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf156a638_7997_48d9_a10c_269d46533f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows_core::HRESULT,
    pub Pairing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationCustomPairing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePairingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPaired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairing2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairing2 {
    type Vtable = IDeviceInformationPairing2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf68612fd_0aee_4328_85cc_1c742bb1790d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnpairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairingStatics {
    type Vtable = IDeviceInformationPairingStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe915c408_36d4_49a1_bf13_514173799b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryRegisterForAllInboundPairingRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairingStatics2 {
    type Vtable = IDeviceInformationPairingStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04de5372_b7b7_476b_a74f_c5836a704d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryRegisterForAllInboundPairingRequestsWithProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationStatics {
    type Vtable = IDeviceInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc17f100e_3a46_4a78_8013_769dc9b97390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncDeviceClass: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilterAndAdditionalProperties: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWatcherDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherAqsFilterAndAdditionalProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationStatics2 {
    type Vtable = IDeviceInformationStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x493b4f34_a84f_45fd_9167_15d1cb1bd1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAqsFilterFromDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncWithKindAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncWithKindAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncWithKindAqsFilterAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, additionalproperties: ::windows_core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithKindAqsFilterAndAdditionalProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationUpdate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationUpdate2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationUpdate2 {
    type Vtable = IDeviceInformationUpdate2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d9d148c_a873_485e_baa6_aa620788e3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PairingKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingKinds) -> ::windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptWithPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingRequestedEventArgs2 {
    type Vtable = IDevicePairingRequestedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc83752d9_e4d3_4db0_a360_a105e437dbdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub AcceptWithPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AcceptWithPasswordCredential: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingResult {
    type Vtable = IDevicePairingResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingResultStatus) -> ::windows_core::HRESULT,
    pub ProtectionLevelUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDevicePairingSettings(::windows_core::IUnknown);
impl IDevicePairingSettings {}
impl ::core::convert::From<IDevicePairingSettings> for ::windows_core::IUnknown {
    fn from(value: IDevicePairingSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows_core::IUnknown {
    fn from(value: &IDevicePairingSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDevicePairingSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDevicePairingSettings> for ::windows_core::IInspectable {
    fn from(value: IDevicePairingSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows_core::IInspectable {
    fn from(value: &IDevicePairingSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IDevicePairingSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDevicePairingSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDevicePairingSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingSettings {}
impl ::core::fmt::Debug for IDevicePairingSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IDevicePairingSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{482cb27c-83bb-420e-be51-6602b222de54}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IDevicePairingSettings {
    type Vtable = IDevicePairingSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x482cb27c_83bb_420e_be51_6602b222de54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePicker {
    type Vtable = IDevicePicker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePicker_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestedProperties: usize,
    pub DeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, placement: ::winrt_ui::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowWithPlacement: usize,
    pub PickSingleDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub PickSingleDeviceAsyncWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: ::winrt_foundation::Rect, placement: ::winrt_ui::Popups::Placement, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PickSingleDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows_core::RawPtr, status: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: DevicePickerDisplayStatusOptions) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePickerAppearance(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerAppearance_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub AccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    AccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_ui::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedAccentColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePickerFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePickerFilter {
    type Vtable = IDevicePickerFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceClasses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceSelectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceSelectors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceSelectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUnpairingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUnpairingResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceUnpairingResultStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcher {
    type Vtable = IDeviceWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcher2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcher2 {
    type Vtable = IDeviceWatcher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff08456e_ed14_49e9_9a69_8117c54ae971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub GetBackgroundTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedeventkinds: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))]
    GetBackgroundTrigger: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherEvent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherEventKind) -> ::windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeviceInformationUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceWatcherEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceWatcherEvents: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnclosureLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnclosureLocation {
    type Vtable = IEnclosureLocation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub InDock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub InLid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Panel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Panel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnclosureLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnclosureLocation2 {
    type Vtable = IEnclosureLocation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2885995b_e07d_485d_8a9e_bdf29aef4f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RotationAngleInDegreesClockwise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Self = Self(0i32);
    pub const Front: Self = Self(1i32);
    pub const Back: Self = Self(2i32);
    pub const Top: Self = Self(3i32);
    pub const Bottom: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl ::core::marker::Copy for Panel {}
impl ::core::clone::Clone for Panel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Panel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Panel {
    type Abi = Self;
}
impl ::core::fmt::Debug for Panel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Panel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Panel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Panel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
