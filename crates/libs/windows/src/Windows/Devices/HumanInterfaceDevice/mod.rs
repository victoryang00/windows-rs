#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControl(::windows_core::IUnknown);
impl HidBooleanControl {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetIsActive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsActive)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ControlDescription(&self) -> ::windows_core::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidBooleanControlDescription>(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControl {}
impl ::core::fmt::Debug for HidBooleanControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidBooleanControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControl;{524df48a-3695-408c-bba2-e2eb5abfbc20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
    const IID: ::windows_core::GUID = <IHidBooleanControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
impl ::core::convert::From<HidBooleanControl> for ::windows_core::IUnknown {
    fn from(value: HidBooleanControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControl> for ::windows_core::IUnknown {
    fn from(value: &HidBooleanControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidBooleanControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidBooleanControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidBooleanControl> for ::windows_core::IInspectable {
    fn from(value: HidBooleanControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControl> for ::windows_core::IInspectable {
    fn from(value: &HidBooleanControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidBooleanControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidBooleanControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidBooleanControl {}
unsafe impl ::core::marker::Sync for HidBooleanControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControlDescription(::windows_core::IUnknown);
impl HidBooleanControlDescription {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).ReportId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportType(&self) -> ::windows_core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HidReportType>::zeroed();
            (::windows_core::Interface::vtable(this).ReportType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentCollections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsAbsolute(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAbsolute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControlDescription {}
impl ::core::fmt::Debug for HidBooleanControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription;{6196e543-29d8-4a2a-8683-849e207bbe31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
    const IID: ::windows_core::GUID = <IHidBooleanControlDescription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
impl ::core::convert::From<HidBooleanControlDescription> for ::windows_core::IUnknown {
    fn from(value: HidBooleanControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControlDescription> for ::windows_core::IUnknown {
    fn from(value: &HidBooleanControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidBooleanControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidBooleanControlDescription> for ::windows_core::IInspectable {
    fn from(value: HidBooleanControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControlDescription> for ::windows_core::IInspectable {
    fn from(value: &HidBooleanControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidBooleanControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidBooleanControlDescription {}
unsafe impl ::core::marker::Sync for HidBooleanControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidCollection(::windows_core::IUnknown);
impl HidCollection {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Type(&self) -> ::windows_core::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HidCollectionType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidCollectionType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for HidCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidCollection {}
impl ::core::fmt::Debug for HidCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidCollection;{7189f5a3-32f1-46e3-befd-44d2663b7e6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidCollection {
    type Vtable = IHidCollection_Vtbl;
    const IID: ::windows_core::GUID = <IHidCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
impl ::core::convert::From<HidCollection> for ::windows_core::IUnknown {
    fn from(value: HidCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidCollection> for ::windows_core::IUnknown {
    fn from(value: &HidCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidCollection> for ::windows_core::IInspectable {
    fn from(value: HidCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidCollection> for ::windows_core::IInspectable {
    fn from(value: &HidCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidCollection {}
unsafe impl ::core::marker::Sync for HidCollection {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for HidCollectionType {}
impl ::core::clone::Clone for HidCollectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidCollectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HidCollectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidCollectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollectionType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidCollectionType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidDevice(::windows_core::IUnknown);
impl HidDevice {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn VendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).VendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Version(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputReportAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetInputReportByIdAsync)(::windows_core::Interface::as_raw(this), reportid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFeatureReportAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFeatureReportByIdAsync)(::windows_core::Interface::as_raw(this), reportid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateOutputReport(&self) -> ::windows_core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOutputReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateOutputReportById(&self, reportid: u16) -> ::windows_core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateOutputReportById)(::windows_core::Interface::as_raw(this), reportid, result__.as_mut_ptr()).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateFeatureReport(&self) -> ::windows_core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFeatureReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateFeatureReportById(&self, reportid: u16) -> ::windows_core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFeatureReportById)(::windows_core::Interface::as_raw(this), reportid, result__.as_mut_ptr()).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendOutputReportAsync<'a, Param0: ::windows_core::IntoParam<'a, HidOutputReport>>(&self, outputreport: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendOutputReportAsync)(::windows_core::Interface::as_raw(this), outputreport.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendFeatureReportAsync<'a, Param0: ::windows_core::IntoParam<'a, HidFeatureReport>>(&self, featurereport: Param0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SendFeatureReportAsync)(::windows_core::Interface::as_raw(this), featurereport.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlDescriptions)(::windows_core::Interface::as_raw(this), reporttype, usagepage, usageid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlDescriptions)(::windows_core::Interface::as_raw(this), reporttype, usagepage, usageid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputReportReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>>(&self, reporthandler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).InputReportReceived)(::windows_core::Interface::as_raw(this), reporthandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputReportReceived<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInputReportReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorVidPid)(::windows_core::Interface::as_raw(this), usagepage, usageid, vendorid, productid, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0, accessmode: super::super::Storage::FileAccessMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), accessmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<HidDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HidDevice, IHidDeviceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HidDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidDevice {}
impl ::core::fmt::Debug for HidDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidDevice {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidDevice;{5f8a14e7-2200-432e-95da-d09b87d574a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidDevice {
    type Vtable = IHidDevice_Vtbl;
    const IID: ::windows_core::GUID = <IHidDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
impl ::core::convert::From<HidDevice> for ::windows_core::IUnknown {
    fn from(value: HidDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidDevice> for ::windows_core::IUnknown {
    fn from(value: &HidDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidDevice> for ::windows_core::IInspectable {
    fn from(value: HidDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidDevice> for ::windows_core::IInspectable {
    fn from(value: &HidDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HidDevice) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HidDevice) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Foundation::IClosable> for &HidDevice {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HidDevice {}
unsafe impl ::core::marker::Sync for HidDevice {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidFeatureReport(::windows_core::IUnknown);
impl HidFeatureReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidFeatureReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidFeatureReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidFeatureReport {}
impl ::core::fmt::Debug for HidFeatureReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidFeatureReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidFeatureReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidFeatureReport;{841d9b79-5ae5-46e3-82ef-1fec5c8942f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
    const IID: ::windows_core::GUID = <IHidFeatureReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
impl ::core::convert::From<HidFeatureReport> for ::windows_core::IUnknown {
    fn from(value: HidFeatureReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidFeatureReport> for ::windows_core::IUnknown {
    fn from(value: &HidFeatureReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidFeatureReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidFeatureReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidFeatureReport> for ::windows_core::IInspectable {
    fn from(value: HidFeatureReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidFeatureReport> for ::windows_core::IInspectable {
    fn from(value: &HidFeatureReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidFeatureReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidFeatureReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidFeatureReport {}
unsafe impl ::core::marker::Sync for HidFeatureReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReport(::windows_core::IUnknown);
impl HidInputReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivatedBooleanControls(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedBooleanControls)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitionedBooleanControls(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransitionedBooleanControls)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReport {}
impl ::core::fmt::Debug for HidInputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidInputReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReport;{c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidInputReport {
    type Vtable = IHidInputReport_Vtbl;
    const IID: ::windows_core::GUID = <IHidInputReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
impl ::core::convert::From<HidInputReport> for ::windows_core::IUnknown {
    fn from(value: HidInputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReport> for ::windows_core::IUnknown {
    fn from(value: &HidInputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidInputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidInputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidInputReport> for ::windows_core::IInspectable {
    fn from(value: HidInputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReport> for ::windows_core::IInspectable {
    fn from(value: &HidInputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidInputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidInputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidInputReport {}
unsafe impl ::core::marker::Sync for HidInputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReportReceivedEventArgs(::windows_core::IUnknown);
impl HidInputReportReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Report(&self) -> ::windows_core::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Report)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidInputReport>(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReportReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReportReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReportReceivedEventArgs {}
impl ::core::fmt::Debug for HidInputReportReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReportReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs;{7059c5cb-59b2-4dc2-985c-0adc6136fa2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IHidInputReportReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
impl ::core::convert::From<HidInputReportReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReportReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidInputReportReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReportReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidInputReportReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HidInputReportReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControl(::windows_core::IUnknown);
impl HidNumericControl {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsGrouped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrouped)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Value(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetValue(&self, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ScaledValue(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).ScaledValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetScaledValue(&self, value: i64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaledValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ControlDescription(&self) -> ::windows_core::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlDescription)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidNumericControlDescription>(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControl {}
impl ::core::fmt::Debug for HidNumericControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControl").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidNumericControl {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControl;{e38a12a5-35a7-4b75-89c8-fb1f28b10823})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
    const IID: ::windows_core::GUID = <IHidNumericControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
impl ::core::convert::From<HidNumericControl> for ::windows_core::IUnknown {
    fn from(value: HidNumericControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControl> for ::windows_core::IUnknown {
    fn from(value: &HidNumericControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidNumericControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidNumericControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidNumericControl> for ::windows_core::IInspectable {
    fn from(value: HidNumericControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControl> for ::windows_core::IInspectable {
    fn from(value: &HidNumericControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidNumericControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidNumericControl {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidNumericControl {}
unsafe impl ::core::marker::Sync for HidNumericControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControlDescription(::windows_core::IUnknown);
impl HidNumericControlDescription {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).ReportId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportType(&self) -> ::windows_core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HidReportType>::zeroed();
            (::windows_core::Interface::vtable(this).ReportType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportSize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ReportCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn LogicalMinimum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).LogicalMinimum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn LogicalMaximum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).LogicalMaximum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn PhysicalMinimum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalMinimum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn PhysicalMaximum(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalMaximum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UnitExponent(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UnitExponent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Unit(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsAbsolute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAbsolute)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn HasNull(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasNull)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentCollections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControlDescription {}
impl ::core::fmt::Debug for HidNumericControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription;{638d5e86-1d97-4c75-927f-5ff58ba05e32})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
    const IID: ::windows_core::GUID = <IHidNumericControlDescription as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
impl ::core::convert::From<HidNumericControlDescription> for ::windows_core::IUnknown {
    fn from(value: HidNumericControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControlDescription> for ::windows_core::IUnknown {
    fn from(value: &HidNumericControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidNumericControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidNumericControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidNumericControlDescription> for ::windows_core::IInspectable {
    fn from(value: HidNumericControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControlDescription> for ::windows_core::IInspectable {
    fn from(value: &HidNumericControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidNumericControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidNumericControlDescription {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidNumericControlDescription {}
unsafe impl ::core::marker::Sync for HidNumericControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidOutputReport(::windows_core::IUnknown);
impl HidOutputReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControl)(::windows_core::Interface::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows_core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows_core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlByDescription)(::windows_core::Interface::as_raw(this), controldescription.into_param().abi(), result__.as_mut_ptr()).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidOutputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidOutputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidOutputReport {}
impl ::core::fmt::Debug for HidOutputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidOutputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidOutputReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidOutputReport;{62cb2544-c896-4463-93c1-df9db053c450})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
    const IID: ::windows_core::GUID = <IHidOutputReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
impl ::core::convert::From<HidOutputReport> for ::windows_core::IUnknown {
    fn from(value: HidOutputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidOutputReport> for ::windows_core::IUnknown {
    fn from(value: &HidOutputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidOutputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidOutputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidOutputReport> for ::windows_core::IInspectable {
    fn from(value: HidOutputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidOutputReport> for ::windows_core::IInspectable {
    fn from(value: &HidOutputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidOutputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidOutputReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidOutputReport {}
unsafe impl ::core::marker::Sync for HidOutputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl ::core::marker::Copy for HidReportType {}
impl ::core::clone::Clone for HidReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HidReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidReportType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidReportType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x524df48a_3695_408c_bba2_e2eb5abfbc20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6196e543_29d8_4a2a_8683_849e207bbe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidBooleanControlDescription2 {
    type Vtable = IHidBooleanControlDescription2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8eed2ea_8a77_4c36_aa00_5ff0449d3e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidCollection {
    type Vtable = IHidCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7189f5a3_32f1_46e3_befd_44d2663b7e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidCollectionType) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidDevice {
    type Vtable = IHidDevice_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f8a14e7_2200_432e_95da_d09b87d574a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDevice_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetInputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInputReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportByIdAsync: usize,
    pub CreateOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateOutputReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFeatureReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendOutputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputreport: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendOutputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featurereport: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendFeatureReportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanControlDescriptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumericControlDescriptions: usize,
    #[cfg(feature = "Foundation")]
    pub InputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputReportReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputReportReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidDeviceStatics {
    type Vtable = IHidDeviceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e5981e4_9856_418c_9f73_77de0cd85754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorVidPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidFeatureReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x841d9b79_5ae5_46e3_82ef_1fec5c8942f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidFeatureReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidInputReport {
    type Vtable = IHidInputReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc35d0e50_f7e7_4e8d_b23e_cabbe56b90e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivatedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivatedBooleanControls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitionedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitionedBooleanControls: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReportReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7059c5cb_59b2_4dc2_985c_0adc6136fa2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe38a12a5_35a7_4b75_89c8_fb1f28b10823);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControl_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows_core::HRESULT,
    pub ScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    pub SetScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows_core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControlDescription(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x638d5e86_1d97_4c75_927f_5ff58ba05e32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControlDescription_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows_core::HRESULT,
    pub ReportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ReportCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub LogicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub LogicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PhysicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PhysicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub UnitExponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidOutputReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62cb2544_c896_4463_93c1_df9db053c450);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidOutputReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
