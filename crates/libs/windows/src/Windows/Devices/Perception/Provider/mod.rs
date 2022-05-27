#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IKnownPerceptionFrameKindStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IKnownPerceptionFrameKindStatics {
    type Vtable = IKnownPerceptionFrameKindStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ae651d6_9669_4106_9fae_4835c1b96104);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IKnownPerceptionFrameKindStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Color: usize,
    #[cfg(feature = "deprecated")]
    pub Depth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Depth: usize,
    #[cfg(feature = "deprecated")]
    pub Infrared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Infrared: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionControlGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionControlGroup {
    type Vtable = IPerceptionControlGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x172c4882_2fd9_4c4e_ba34_fdf20a73dde5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FrameProviderIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FrameProviderIds: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionControlGroupFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionControlGroupFactory {
    type Vtable = IPerceptionControlGroupFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f1af2e0_baf1_453b_bed4_cd9d4619154c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionControlGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ids: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionCorrelation(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionCorrelation {
    type Vtable = IPerceptionCorrelation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4131a82_dff5_4047_8a19_3b4d805f7176);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub TargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TargetId: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Position: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Orientation: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionCorrelationFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionCorrelationFactory {
    type Vtable = IPerceptionCorrelationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4a6c425_2884_4a8f_8134_2835d7286cbf);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, position: super::super::super::Foundation::Numerics::Vector3, orientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionCorrelationGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x752a0906_36a7_47bb_9b79_56cc6b746770);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub RelativeLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    RelativeLocations: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionCorrelationGroupFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionCorrelationGroupFactory {
    type Vtable = IPerceptionCorrelationGroupFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dfe2088_63df_48ed_83b1_4ab829132995);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionCorrelationGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativelocations: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8019814_4a91_41b0_83a6_881a1775353e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FrameProviderIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FrameProviderIds: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroupFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFaceAuthenticationGroupFactory {
    type Vtable = IPerceptionFaceAuthenticationGroupFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe68a05d4_b60c_40f4_bcb9_f24d46467320);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFaceAuthenticationGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ids: ::windows_core::RawPtr, starthandler: ::windows_core::RawPtr, stophandler: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Create: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrame(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFrame {
    type Vtable = IPerceptionFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cfe7825_54bb_4d9d_bec5_8ef66151d2ac);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RelativeTime: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetRelativeTime: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FrameData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FrameData: usize,
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameProvider(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IPerceptionFrameProvider {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameProviderInfo(&self) -> ::windows_core::Result<PerceptionFrameProviderInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameProviderInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionFrameProviderInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Available(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Available)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, PerceptionPropertyChangeRequest>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProperty)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IPerceptionFrameProvider> for ::windows_core::IUnknown {
    fn from(value: IPerceptionFrameProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IPerceptionFrameProvider> for ::windows_core::IUnknown {
    fn from(value: &IPerceptionFrameProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IPerceptionFrameProvider> for ::windows_core::IInspectable {
    fn from(value: IPerceptionFrameProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IPerceptionFrameProvider> for ::windows_core::IInspectable {
    fn from(value: &IPerceptionFrameProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IPerceptionFrameProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&IPerceptionFrameProvider> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPerceptionFrameProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProvider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IPerceptionFrameProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IPerceptionFrameProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IPerceptionFrameProvider {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IPerceptionFrameProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerceptionFrameProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IPerceptionFrameProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{794f7ab9-b37d-3b33-a10d-30626419ce65}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFrameProvider {
    type Vtable = IPerceptionFrameProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x794f7ab9_b37d_3b33_a10d_30626419ce65);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub FrameProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub Available: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Available: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
    #[cfg(feature = "deprecated")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetProperty: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameProviderInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcca959e8_797e_4e83_9b87_036a74142fc4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetId: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDisplayName: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetDeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub FrameKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetFrameKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetFrameKind: usize,
    #[cfg(feature = "deprecated")]
    pub Hidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Hidden: usize,
    #[cfg(feature = "deprecated")]
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetHidden: usize,
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameProviderManager(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IPerceptionFrameProviderManager {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetFrameProvider<'a, Param0: ::windows_core::IntoParam<'a, PerceptionFrameProviderInfo>>(&self, frameproviderinfo: Param0) -> ::windows_core::Result<IPerceptionFrameProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFrameProvider)(::windows_core::Interface::as_raw(this), frameproviderinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<IPerceptionFrameProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IPerceptionFrameProviderManager> for ::windows_core::IUnknown {
    fn from(value: IPerceptionFrameProviderManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IPerceptionFrameProviderManager> for ::windows_core::IUnknown {
    fn from(value: &IPerceptionFrameProviderManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IPerceptionFrameProviderManager> for ::windows_core::IInspectable {
    fn from(value: IPerceptionFrameProviderManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IPerceptionFrameProviderManager> for ::windows_core::IInspectable {
    fn from(value: &IPerceptionFrameProviderManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: IPerceptionFrameProviderManager) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&IPerceptionFrameProviderManager> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPerceptionFrameProviderManager) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IPerceptionFrameProviderManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IPerceptionFrameProviderManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IPerceptionFrameProviderManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IPerceptionFrameProviderManager {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IPerceptionFrameProviderManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerceptionFrameProviderManager").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IPerceptionFrameProviderManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a959ce07-ead3-33df-8ec1-b924abe019c4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFrameProviderManager {
    type Vtable = IPerceptionFrameProviderManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa959ce07_ead3_33df_8ec1_b924abe019c4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetFrameProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetFrameProvider: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionFrameProviderManagerServiceStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionFrameProviderManagerServiceStatics {
    type Vtable = IPerceptionFrameProviderManagerServiceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae8386e6_cad9_4359_8f96_8eae51810526);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionFrameProviderManagerServiceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub RegisterFrameProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, frameproviderinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterFrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterFrameProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, frameproviderinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterFrameProviderInfo: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterFaceAuthenticationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, faceauthenticationgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterFaceAuthenticationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterFaceAuthenticationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, faceauthenticationgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterFaceAuthenticationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterControlGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, controlgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterControlGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterControlGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, controlgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterControlGroup: usize,
    #[cfg(feature = "deprecated")]
    pub RegisterCorrelationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, correlationgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegisterCorrelationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UnregisterCorrelationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manager: ::windows_core::RawPtr, correlationgroup: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UnregisterCorrelationGroup: usize,
    #[cfg(feature = "deprecated")]
    pub UpdateAvailabilityForProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, available: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UpdateAvailabilityForProvider: usize,
    #[cfg(feature = "deprecated")]
    pub PublishFrameForProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, frame: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PublishFrameForProvider: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionPropertyChangeRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequest_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c5aeb51_350b_4df8_9414_59e09815510b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionPropertyChangeRequest_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(feature = "deprecated")]
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetStatus: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocator(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c38a7da_fdd8_4ed4_a039_2a6f9b235038);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AllocateFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AllocateFrame: usize,
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub CopyFromVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media", feature = "deprecated")))]
    CopyFromVideoFrame: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocatorFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPerceptionVideoFrameAllocatorFactory {
    type Vtable = IPerceptionVideoFrameAllocatorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a58b0e1_e91a_481e_b876_a89e2bbc6b33);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerceptionVideoFrameAllocatorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated")))]
    Create: usize,
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct KnownPerceptionFrameKind;
#[cfg(feature = "deprecated")]
impl KnownPerceptionFrameKind {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Color() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Depth() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Depth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Infrared() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownPerceptionFrameKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Infrared)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IKnownPerceptionFrameKindStatics<R, F: FnOnce(&IKnownPerceptionFrameKindStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownPerceptionFrameKind, IKnownPerceptionFrameKindStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for KnownPerceptionFrameKind {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.KnownPerceptionFrameKind";
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionControlGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionControlGroup {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FrameProviderIds(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameProviderIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>>(ids: Param0) -> ::windows_core::Result<PerceptionControlGroup> {
        Self::IPerceptionControlGroupFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ids.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionControlGroup>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionControlGroupFactory<R, F: FnOnce(&IPerceptionControlGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionControlGroup, IPerceptionControlGroupFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionControlGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionControlGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionControlGroup {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionControlGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionControlGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionControlGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionControlGroup;{172c4882-2fd9-4c4e-ba34-fdf20a73dde5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionControlGroup {
    type Vtable = IPerceptionControlGroup_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionControlGroup as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionControlGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionControlGroup";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionControlGroup> for ::windows_core::IUnknown {
    fn from(value: PerceptionControlGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlGroup> for ::windows_core::IUnknown {
    fn from(value: &PerceptionControlGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionControlGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionControlGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionControlGroup> for ::windows_core::IInspectable {
    fn from(value: PerceptionControlGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionControlGroup> for ::windows_core::IInspectable {
    fn from(value: &PerceptionControlGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionControlGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionControlGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionControlGroup {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionControlGroup {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionCorrelation(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionCorrelation {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TargetId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn Position(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn Orientation(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Numerics\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(targetid: Param0, position: Param1, orientation: Param2) -> ::windows_core::Result<PerceptionCorrelation> {
        Self::IPerceptionCorrelationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), targetid.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionCorrelation>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionCorrelationFactory<R, F: FnOnce(&IPerceptionCorrelationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionCorrelation, IPerceptionCorrelationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionCorrelation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionCorrelation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionCorrelation {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionCorrelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionCorrelation").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionCorrelation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelation;{b4131a82-dff5-4047-8a19-3b4d805f7176})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionCorrelation {
    type Vtable = IPerceptionCorrelation_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionCorrelation as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionCorrelation {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelation";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionCorrelation> for ::windows_core::IUnknown {
    fn from(value: PerceptionCorrelation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionCorrelation> for ::windows_core::IUnknown {
    fn from(value: &PerceptionCorrelation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionCorrelation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionCorrelation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionCorrelation> for ::windows_core::IInspectable {
    fn from(value: PerceptionCorrelation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionCorrelation> for ::windows_core::IInspectable {
    fn from(value: &PerceptionCorrelation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionCorrelation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionCorrelation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionCorrelation {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionCorrelation {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionCorrelationGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionCorrelationGroup {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RelativeLocations(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeLocations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>>(relativelocations: Param0) -> ::windows_core::Result<PerceptionCorrelationGroup> {
        Self::IPerceptionCorrelationGroupFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), relativelocations.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionCorrelationGroup>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionCorrelationGroupFactory<R, F: FnOnce(&IPerceptionCorrelationGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionCorrelationGroup, IPerceptionCorrelationGroupFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionCorrelationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionCorrelationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionCorrelationGroup {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionCorrelationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionCorrelationGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionCorrelationGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionCorrelationGroup;{752a0906-36a7-47bb-9b79-56cc6b746770})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionCorrelationGroup {
    type Vtable = IPerceptionCorrelationGroup_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionCorrelationGroup as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionCorrelationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionCorrelationGroup";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionCorrelationGroup> for ::windows_core::IUnknown {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionCorrelationGroup> for ::windows_core::IUnknown {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionCorrelationGroup> for ::windows_core::IInspectable {
    fn from(value: PerceptionCorrelationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionCorrelationGroup> for ::windows_core::IInspectable {
    fn from(value: &PerceptionCorrelationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionCorrelationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionCorrelationGroup {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionCorrelationGroup {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFaceAuthenticationGroup(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionFaceAuthenticationGroup {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FrameProviderIds(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameProviderIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>, Param1: ::windows_core::IntoParam<'a, PerceptionStartFaceAuthenticationHandler>, Param2: ::windows_core::IntoParam<'a, PerceptionStopFaceAuthenticationHandler>>(ids: Param0, starthandler: Param1, stophandler: Param2) -> ::windows_core::Result<PerceptionFaceAuthenticationGroup> {
        Self::IPerceptionFaceAuthenticationGroupFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ids.into_param().abi(), starthandler.into_param().abi(), stophandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionFaceAuthenticationGroup>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionFaceAuthenticationGroupFactory<R, F: FnOnce(&IPerceptionFaceAuthenticationGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionFaceAuthenticationGroup, IPerceptionFaceAuthenticationGroupFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFaceAuthenticationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionFaceAuthenticationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionFaceAuthenticationGroup {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFaceAuthenticationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFaceAuthenticationGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionFaceAuthenticationGroup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup;{e8019814-4a91-41b0-83a6-881a1775353e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionFaceAuthenticationGroup {
    type Vtable = IPerceptionFaceAuthenticationGroup_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionFaceAuthenticationGroup as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionFaceAuthenticationGroup {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFaceAuthenticationGroup";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFaceAuthenticationGroup> for ::windows_core::IUnknown {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows_core::IUnknown {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFaceAuthenticationGroup> for ::windows_core::IInspectable {
    fn from(value: PerceptionFaceAuthenticationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFaceAuthenticationGroup> for ::windows_core::IInspectable {
    fn from(value: &PerceptionFaceAuthenticationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionFaceAuthenticationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionFaceAuthenticationGroup {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionFaceAuthenticationGroup {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrame(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionFrame {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetRelativeTime<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FrameData(&self) -> ::windows_core::Result<super::super::super::Foundation::IMemoryBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FrameData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IMemoryBuffer>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionFrame {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrame;{7cfe7825-54bb-4d9d-bec5-8ef66151d2ac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionFrame {
    type Vtable = IPerceptionFrame_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionFrame as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionFrame {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrame";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrame> for ::windows_core::IUnknown {
    fn from(value: PerceptionFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrame> for ::windows_core::IUnknown {
    fn from(value: &PerceptionFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrame> for ::windows_core::IInspectable {
    fn from(value: PerceptionFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrame> for ::windows_core::IInspectable {
    fn from(value: &PerceptionFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionFrame {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionFrame {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionFrameProviderInfo(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionFrameProviderInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionFrameProviderInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn DeviceKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetDeviceKind<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDeviceKind)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn FrameKind(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).FrameKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetFrameKind<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrameKind)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Hidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Hidden)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHidden)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionFrameProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionFrameProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionFrameProviderInfo {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionFrameProviderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionFrameProviderInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionFrameProviderInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo;{cca959e8-797e-4e83-9b87-036a74142fc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionFrameProviderInfo {
    type Vtable = IPerceptionFrameProviderInfo_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionFrameProviderInfo as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionFrameProviderInfo {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderInfo";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameProviderInfo> for ::windows_core::IUnknown {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameProviderInfo> for ::windows_core::IUnknown {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionFrameProviderInfo> for ::windows_core::IInspectable {
    fn from(value: PerceptionFrameProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionFrameProviderInfo> for ::windows_core::IInspectable {
    fn from(value: &PerceptionFrameProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionFrameProviderInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionFrameProviderInfo {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionFrameProviderInfo {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct PerceptionFrameProviderManagerService;
#[cfg(feature = "deprecated")]
impl PerceptionFrameProviderManagerService {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RegisterFrameProviderInfo<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterFrameProviderInfo)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UnregisterFrameProviderInfo<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionFrameProviderInfo>>(manager: Param0, frameproviderinfo: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterFrameProviderInfo)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), frameproviderinfo.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RegisterFaceAuthenticationGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterFaceAuthenticationGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UnregisterFaceAuthenticationGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(manager: Param0, faceauthenticationgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterFaceAuthenticationGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), faceauthenticationgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RegisterControlGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterControlGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UnregisterControlGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionControlGroup>>(manager: Param0, controlgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterControlGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), controlgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn RegisterCorrelationGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCorrelationGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UnregisterCorrelationGroup<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProviderManager>, Param1: ::windows_core::IntoParam<'a, PerceptionCorrelationGroup>>(manager: Param0, correlationgroup: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterCorrelationGroup)(::windows_core::Interface::as_raw(this), manager.into_param().abi(), correlationgroup.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UpdateAvailabilityForProvider<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProvider>>(provider: Param0, available: bool) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).UpdateAvailabilityForProvider)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), available).ok() })
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PublishFrameForProvider<'a, Param0: ::windows_core::IntoParam<'a, IPerceptionFrameProvider>, Param1: ::windows_core::IntoParam<'a, PerceptionFrame>>(provider: Param0, frame: Param1) -> ::windows_core::Result<()> {
        Self::IPerceptionFrameProviderManagerServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).PublishFrameForProvider)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), frame.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionFrameProviderManagerServiceStatics<R, F: FnOnce(&IPerceptionFrameProviderManagerServiceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionFrameProviderManagerService, IPerceptionFrameProviderManagerServiceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionFrameProviderManagerService {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionFrameProviderManagerService";
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionPropertyChangeRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionPropertyChangeRequest {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Status(&self) -> ::windows_core::Result<super::PerceptionFrameSourcePropertyChangeStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::PerceptionFrameSourcePropertyChangeStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PerceptionFrameSourcePropertyChangeStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetStatus(&self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionPropertyChangeRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionPropertyChangeRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionPropertyChangeRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionPropertyChangeRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionPropertyChangeRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionPropertyChangeRequest {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest;{3c5aeb51-350b-4df8-9414-59e09815510b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionPropertyChangeRequest {
    type Vtable = IPerceptionPropertyChangeRequest_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionPropertyChangeRequest as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionPropertyChangeRequest {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionPropertyChangeRequest";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionPropertyChangeRequest> for ::windows_core::IUnknown {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionPropertyChangeRequest> for ::windows_core::IUnknown {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionPropertyChangeRequest> for ::windows_core::IInspectable {
    fn from(value: PerceptionPropertyChangeRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionPropertyChangeRequest> for ::windows_core::IInspectable {
    fn from(value: &PerceptionPropertyChangeRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionPropertyChangeRequest {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionPropertyChangeRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionPropertyChangeRequest {}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionStartFaceAuthenticationHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionStartFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PerceptionStartFaceAuthenticationHandlerBox::<F> { vtable: &PerceptionStartFaceAuthenticationHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct PerceptionStartFaceAuthenticationHandlerBox<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static> {
    vtable: *const PerceptionStartFaceAuthenticationHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static> PerceptionStartFaceAuthenticationHandlerBox<F> {
    const VTABLE: PerceptionStartFaceAuthenticationHandler_Vtbl = PerceptionStartFaceAuthenticationHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStartFaceAuthenticationHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        match ((*this).invoke)(::core::mem::transmute(&sender)) {
            ::core::result::Result::Ok(ok__) => {
                ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                ::core::mem::forget(ok__);
                ::windows_core::HRESULT(0)
            }
            ::core::result::Result::Err(err) => err.into(),
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionStartFaceAuthenticationHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionStartFaceAuthenticationHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionStartFaceAuthenticationHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionStartFaceAuthenticationHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionStartFaceAuthenticationHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionStartFaceAuthenticationHandler {
    type Vtable = PerceptionStartFaceAuthenticationHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74816d2a_2090_4670_8c48_ef39e7ff7c26);
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionStartFaceAuthenticationHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{74816d2a-2090-4670-8c48-ef39e7ff7c26}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStartFaceAuthenticationHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionStopFaceAuthenticationHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionStopFaceAuthenticationHandler {
    pub fn new<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PerceptionStopFaceAuthenticationHandlerBox::<F> { vtable: &PerceptionStopFaceAuthenticationHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, PerceptionFaceAuthenticationGroup>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct PerceptionStopFaceAuthenticationHandlerBox<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PerceptionStopFaceAuthenticationHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(&::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> PerceptionStopFaceAuthenticationHandlerBox<F> {
    const VTABLE: PerceptionStopFaceAuthenticationHandler_Vtbl = PerceptionStopFaceAuthenticationHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<PerceptionStopFaceAuthenticationHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionStopFaceAuthenticationHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionStopFaceAuthenticationHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionStopFaceAuthenticationHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionStopFaceAuthenticationHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionStopFaceAuthenticationHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionStopFaceAuthenticationHandler {
    type Vtable = PerceptionStopFaceAuthenticationHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x387ee6aa_89cd_481e_aade_dd92f70b2ad7);
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionStopFaceAuthenticationHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{387ee6aa-89cd-481e-aade-dd92f70b2ad7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct PerceptionStopFaceAuthenticationHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PerceptionVideoFrameAllocator(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PerceptionVideoFrameAllocator {
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AllocateFrame(&self) -> ::windows_core::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllocateFrame)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media", feature = "deprecated"))]
    pub fn CopyFromVideoFrame<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Media::VideoFrame>>(&self, frame: Param0) -> ::windows_core::Result<PerceptionFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CopyFromVideoFrame)(::windows_core::Interface::as_raw(this), frame.into_param().abi(), result__.as_mut_ptr()).from_abi::<PerceptionFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Perception_Provider\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "deprecated"))]
    pub fn Create<'a, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::Size>>(maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: Param2, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_core::Result<PerceptionVideoFrameAllocator> {
        Self::IPerceptionVideoFrameAllocatorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), maxoutstandingframecountforwrite, format, resolution.into_param().abi(), alpha, result__.as_mut_ptr()).from_abi::<PerceptionVideoFrameAllocator>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPerceptionVideoFrameAllocatorFactory<R, F: FnOnce(&IPerceptionVideoFrameAllocatorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PerceptionVideoFrameAllocator, IPerceptionVideoFrameAllocatorFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PerceptionVideoFrameAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PerceptionVideoFrameAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PerceptionVideoFrameAllocator {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PerceptionVideoFrameAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PerceptionVideoFrameAllocator").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for PerceptionVideoFrameAllocator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator;{4c38a7da-fdd8-4ed4-a039-2a6f9b235038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PerceptionVideoFrameAllocator {
    type Vtable = IPerceptionVideoFrameAllocator_Vtbl;
    const IID: ::windows_core::GUID = <IPerceptionVideoFrameAllocator as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PerceptionVideoFrameAllocator {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.PerceptionVideoFrameAllocator";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionVideoFrameAllocator> for ::windows_core::IUnknown {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoFrameAllocator> for ::windows_core::IUnknown {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PerceptionVideoFrameAllocator> for ::windows_core::IInspectable {
    fn from(value: PerceptionVideoFrameAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PerceptionVideoFrameAllocator> for ::windows_core::IInspectable {
    fn from(value: &PerceptionVideoFrameAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: PerceptionVideoFrameAllocator) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&PerceptionVideoFrameAllocator> for super::super::super::Foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &PerceptionVideoFrameAllocator) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Foundation::IClosable> for &PerceptionVideoFrameAllocator {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PerceptionVideoFrameAllocator {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PerceptionVideoFrameAllocator {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
