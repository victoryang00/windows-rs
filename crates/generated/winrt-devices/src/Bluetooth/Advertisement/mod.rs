#[repr(transparent)]
pub struct BluetoothLEAdvertisement(::windows_core::IUnknown);
impl BluetoothLEAdvertisement {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisement, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Flags(&self) -> ::windows_core::Result<::winrt_foundation::IReference<BluetoothLEAdvertisementFlags>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Flags)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<BluetoothLEAdvertisementFlags>>(result__)
        }
    }
    pub fn SetFlags<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<BluetoothLEAdvertisementFlags>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlags)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LocalName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLocalName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLocalName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ServiceUuids(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::GUID>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ManufacturerData(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn DataSections(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DataSections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetManufacturerDataByCompanyId)(::windows_core::Interface::as_raw(this), companyid, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetSectionsByType(&self, r#type: u8) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSectionsByType)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisement {}
impl ::core::fmt::Debug for BluetoothLEAdvertisement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement;{066fb2b7-33d1-4e7d-8367-cf81d0f79653})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisement as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisement {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisement {}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementBytePattern(::windows_core::IUnknown);
impl BluetoothLEAdvertisementBytePattern {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementBytePattern, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DataType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).DataType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Offset(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn SetOffset(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Create<'a, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(datatype: u8, offset: i16, data: Param2) -> ::windows_core::Result<BluetoothLEAdvertisementBytePattern> {
        Self::IBluetoothLEAdvertisementBytePatternFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), datatype, offset, data.into_param().abi(), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementBytePattern>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementBytePatternFactory<R, F: FnOnce(&IBluetoothLEAdvertisementBytePatternFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePatternFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementBytePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementBytePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementBytePattern {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementBytePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementBytePattern").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementBytePattern {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern;{fbfad7f2-b9c5-4a08-bc51-502f8ef68a79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementBytePattern as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementBytePattern {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementBytePattern {}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementDataSection(::windows_core::IUnknown);
impl BluetoothLEAdvertisementDataSection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementDataSection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DataType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).DataType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Create<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(datatype: u8, data: Param1) -> ::windows_core::Result<BluetoothLEAdvertisementDataSection> {
        Self::IBluetoothLEAdvertisementDataSectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), datatype, data.into_param().abi(), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementDataSection>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataSectionFactory<R, F: FnOnce(&IBluetoothLEAdvertisementDataSectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSectionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementDataSection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementDataSection {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementDataSection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementDataSection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementDataSection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection;{d7213314-3a43-40f9-b6f0-92bfefc34ae3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementDataSection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementDataSection {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementDataSection {}
pub struct BluetoothLEAdvertisementDataTypes;
impl BluetoothLEAdvertisementDataTypes {
    pub fn Flags() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).Flags)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService16BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).IncompleteService16BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService16BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteService16BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService32BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).IncompleteService32BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService32BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteService32BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService128BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).IncompleteService128BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService128BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteService128BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ShortenedLocalName() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ShortenedLocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteLocalName() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).CompleteLocalName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn TxPowerLevel() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).TxPowerLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn SlaveConnectionIntervalRange() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).SlaveConnectionIntervalRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation16BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceSolicitation16BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation32BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceSolicitation32BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation128BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceSolicitation128BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData16BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceData16BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData32BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceData32BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData128BitUuids() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceData128BitUuids)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn PublicTargetAddress() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).PublicTargetAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn RandomTargetAddress() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).RandomTargetAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn Appearance() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).Appearance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn AdvertisingInterval() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn ManufacturerSpecificData() -> ::windows_core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerSpecificData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataTypesStatics<R, F: FnOnce(&IBluetoothLEAdvertisementDataTypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementDataTypes, IBluetoothLEAdvertisementDataTypesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementDataTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFilter(::windows_core::IUnknown);
impl BluetoothLEAdvertisementFilter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementFilter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Advertisement(&self) -> ::windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    pub fn SetAdvertisement<'a, Param0: ::windows_core::IntoParam<'a, BluetoothLEAdvertisement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAdvertisement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BytePatterns(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BytePatterns)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementFilter {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementFilter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter;{131eb0d3-d04e-47b1-837e-49405bf6f80f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementFilter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementFilter {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementFilter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const GeneralDiscoverableMode: Self = Self(2u32);
    pub const ClassicNotSupported: Self = Self(4u32);
    pub const DualModeControllerCapable: Self = Self(8u32);
    pub const DualModeHostCapable: Self = Self(16u32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementFlags {}
impl ::core::clone::Clone for BluetoothLEAdvertisementFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BluetoothLEAdvertisementFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementFlags").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BluetoothLEAdvertisementFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BluetoothLEAdvertisementFlags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementFlags {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisher(::windows_core::IUnknown);
impl BluetoothLEAdvertisementPublisher {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementPublisher, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Status(&self) -> ::windows_core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BluetoothLEAdvertisementPublisherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisement>(result__)
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
    pub fn StatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStatusChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i16>> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredTransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i16>>(result__)
        }
    }
    pub fn SetPreferredTransmitPowerLevelInDBm<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<i16>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredTransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UseExtendedAdvertisement(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).UseExtendedAdvertisement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUseExtendedAdvertisement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnonymous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAnonymous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeTransmitPowerLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeTransmitPowerLevel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, BluetoothLEAdvertisement>>(advertisement: Param0) -> ::windows_core::Result<BluetoothLEAdvertisementPublisher> {
        Self::IBluetoothLEAdvertisementPublisherFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), advertisement.into_param().abi(), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementPublisher>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementPublisherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementPublisherFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisherFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisher {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementPublisher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher;{cde820f9-d9fa-43d6-a264-ddd8b7da8b78})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementPublisher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Waiting: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementPublisherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BluetoothLEAdvertisementPublisherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementPublisherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
impl BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BluetoothLEAdvertisementPublisherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    pub fn Error(&self) -> ::windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::BluetoothError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothError>(result__)
        }
    }
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i16>> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedTransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i16>>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs;{09c2bd9f-2dff-4b23-86ee-0d14fb94aeae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementPublisherStatusChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(::windows_core::IUnknown);
impl BluetoothLEAdvertisementReceivedEventArgs {
    pub fn RawSignalStrengthInDBm(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
            (::windows_core::Interface::vtable(this).RawSignalStrengthInDBm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
            (::windows_core::Interface::vtable(this).BluetoothAddress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn AdvertisementType(&self) -> ::windows_core::Result<BluetoothLEAdvertisementType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BluetoothLEAdvertisementType>::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisementType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementType>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows_core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    pub fn BluetoothAddressType(&self) -> ::windows_core::Result<super::BluetoothAddressType> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::BluetoothAddressType>::zeroed();
            (::windows_core::Interface::vtable(this).BluetoothAddressType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothAddressType>(result__)
        }
    }
    pub fn TransmitPowerLevelInDBm(&self) -> ::windows_core::Result<::winrt_foundation::IReference<i16>> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<i16>>(result__)
        }
    }
    pub fn IsAnonymous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnonymous)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsConnectable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnectable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsScannable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScannable)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsDirected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDirected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsScanResponse(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScanResponse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementReceivedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs;{27987ddf-e596-41be-8d43-9e6731d4a913})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementReceivedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: Self = Self(0i32);
    pub const ConnectableDirected: Self = Self(1i32);
    pub const ScannableUndirected: Self = Self(2i32);
    pub const NonConnectableUndirected: Self = Self(3i32);
    pub const ScanResponse: Self = Self(4i32);
    pub const Extended: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementType {}
impl ::core::clone::Clone for BluetoothLEAdvertisementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BluetoothLEAdvertisementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcher(::windows_core::IUnknown);
impl BluetoothLEAdvertisementWatcher {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementWatcher, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MinSamplingInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MinSamplingInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MaxSamplingInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSamplingInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MinOutOfRangeTimeout(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MinOutOfRangeTimeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).MaxOutOfRangeTimeout)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<BluetoothLEAdvertisementWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BluetoothLEAdvertisementWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementWatcherStatus>(result__)
        }
    }
    pub fn ScanningMode(&self) -> ::windows_core::Result<BluetoothLEScanningMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BluetoothLEScanningMode>::zeroed();
            (::windows_core::Interface::vtable(this).ScanningMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEScanningMode>(result__)
        }
    }
    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScanningMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SignalStrengthFilter(&self) -> ::windows_core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignalStrengthFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
    pub fn SetSignalStrengthFilter<'a, Param0: ::windows_core::IntoParam<'a, super::BluetoothSignalStrengthFilter>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalStrengthFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AdvertisementFilter(&self) -> ::windows_core::Result<BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisementFilter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementFilter>(result__)
        }
    }
    pub fn SetAdvertisementFilter<'a, Param0: ::windows_core::IntoParam<'a, BluetoothLEAdvertisementFilter>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAdvertisementFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Received<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Received)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveReceived<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReceived)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Stopped<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn AllowExtendedAdvertisements(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowExtendedAdvertisements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowExtendedAdvertisements)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, BluetoothLEAdvertisementFilter>>(advertisementfilter: Param0) -> ::windows_core::Result<BluetoothLEAdvertisementWatcher> {
        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), advertisementfilter.into_param().abi(), result__.as_mut_ptr()).from_abi::<BluetoothLEAdvertisementWatcher>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementWatcherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcherFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcher {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher;{a6ac336f-f3d3-4297-8d6c-c81ea6623f40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopping: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BluetoothLEAdvertisementWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(::windows_core::IUnknown);
impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::BluetoothError>::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::BluetoothError>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherStoppedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs;{dd40f84d-e7b9-43e3-9c04-0685d085fd8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementWatcherStoppedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
#[repr(transparent)]
pub struct BluetoothLEManufacturerData(::windows_core::IUnknown);
impl BluetoothLEManufacturerData {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEManufacturerData, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CompanyId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).CompanyId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetCompanyId(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompanyId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Create<'a, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IBuffer>>(companyid: u16, data: Param1) -> ::windows_core::Result<BluetoothLEManufacturerData> {
        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), companyid, data.into_param().abi(), result__.as_mut_ptr()).from_abi::<BluetoothLEManufacturerData>(result__)
        })
    }
    pub fn IBluetoothLEManufacturerDataFactory<R, F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEManufacturerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEManufacturerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEManufacturerData {}
impl ::core::fmt::Debug for BluetoothLEManufacturerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEManufacturerData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEManufacturerData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData;{912dba18-6963-4533-b061-4694dafb34e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_Vtbl;
    const IID: ::windows_core::GUID = <IBluetoothLEManufacturerData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows_core::IUnknown {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows_core::IUnknown {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows_core::IInspectable {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows_core::IInspectable {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEManufacturerData {}
unsafe impl ::core::marker::Sync for BluetoothLEManufacturerData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothLEScanningMode {}
impl ::core::clone::Clone for BluetoothLEScanningMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEScanningMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BluetoothLEScanningMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEScanningMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEScanningMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BluetoothLEScanningMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub ServiceUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ServiceUuids: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ManufacturerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ManufacturerData: usize,
    #[cfg(feature = "winrt-foundation")]
    pub DataSections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    DataSections: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetManufacturerDataByCompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, companyid: u16, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetManufacturerDataByCompanyId: usize,
    #[cfg(feature = "winrt-foundation")]
    pub GetSectionsByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetSectionsByType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePattern(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementBytePatternFactory {
    type Vtable = IBluetoothLEAdvertisementBytePatternFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2e24d73_fd5c_4ec3_be2a_9ca6fa11b7bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: u8, offset: i16, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementDataSectionFactory {
    type Vtable = IBluetoothLEAdvertisementDataSectionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7a40942_a845_4045_bf7e_3e9971db8a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: u8, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementDataTypesStatics {
    type Vtable = IBluetoothLEAdvertisementDataTypesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bb6472f_0606_434b_a76e_74159f0684d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub IncompleteService16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub CompleteService16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub IncompleteService32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub CompleteService32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub IncompleteService128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub CompleteService128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ShortenedLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub CompleteLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SlaveConnectionIntervalRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceSolicitation16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceSolicitation32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceSolicitation128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceData16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceData32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ServiceData128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub PublicTargetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub RandomTargetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub AdvertisingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub ManufacturerSpecificData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub BytePatterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BytePatterns: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisher2 {
    type Vtable = IBluetoothLEAdvertisementPublisher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbdb545e_56f1_510f_a434_217fbd9e7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UseExtendedAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUseExtendedAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisherFactory {
    type Vtable = IBluetoothLEAdvertisementPublisherFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c5f065e_b863_4981_a1af_1c544d8b0c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advertisement: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f62790e_dc88_5c8b_b34e_10b321850f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RawSignalStrengthInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AdvertisementType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementType) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementReceivedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12d9c87b_0399_5f0e_a348_53b02b6b162e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BluetoothAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothAddressType) -> ::windows_core::HRESULT,
    pub TransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsScannable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDirected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsScanResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows_core::HRESULT,
    pub ScanningMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEScanningMode) -> ::windows_core::HRESULT,
    pub SetScanningMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BluetoothLEScanningMode) -> ::windows_core::HRESULT,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Received: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcher2 {
    type Vtable = IBluetoothLEAdvertisementWatcher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01bf26bc_b164_5805_90a3_e8a7997ff225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcherFactory {
    type Vtable = IBluetoothLEAdvertisementWatcherFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9aaf2d56_39ac_453e_b32a_85c657e017f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advertisementfilter: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEManufacturerData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetCompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
    #[cfg(feature = "winrt-storage")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEManufacturerDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEManufacturerDataFactory {
    type Vtable = IBluetoothLEManufacturerDataFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc09b39f8_319a_441e_8de5_66a81e877a6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, companyid: u16, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Create: usize,
}
