#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8271b23_735f_5729_a98e_e64ed189abc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropFrameOfReferencePreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NodeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CoordinateSystemToNodeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CoordinateSystemToNodeTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialGraphInteropPreviewStatics {
    type Vtable = ISpatialGraphInteropPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc042644c_20d8_4ed0_aef7_6805b8e53f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateCoordinateSystemForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows_core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows_core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPositionAndOrientation: usize,
    pub CreateLocatorForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows_core::GUID, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialGraphInteropPreviewStatics2 {
    type Vtable = ISpatialGraphInteropPreviewStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2490b15f_6cbd_4b1e_b765_31e462a32df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreateFrameOfReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPositionAndOrientation: usize,
}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
#[repr(transparent)]
pub struct SpatialGraphInteropFrameOfReferencePreview(::windows_core::IUnknown);
impl SpatialGraphInteropFrameOfReferencePreview {
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn NodeId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).NodeId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CoordinateSystemToNodeTransform(&self) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Numerics::Matrix4x4>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystemToNodeTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialGraphInteropFrameOfReferencePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialGraphInteropFrameOfReferencePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialGraphInteropFrameOfReferencePreview {}
impl ::core::fmt::Debug for SpatialGraphInteropFrameOfReferencePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialGraphInteropFrameOfReferencePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview;{a8271b23-735f-5729-a98e-e64ed189abc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialGraphInteropFrameOfReferencePreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview";
}
impl ::core::convert::From<SpatialGraphInteropFrameOfReferencePreview> for ::windows_core::IUnknown {
    fn from(value: SpatialGraphInteropFrameOfReferencePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGraphInteropFrameOfReferencePreview> for ::windows_core::IUnknown {
    fn from(value: &SpatialGraphInteropFrameOfReferencePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialGraphInteropFrameOfReferencePreview> for ::windows_core::IInspectable {
    fn from(value: SpatialGraphInteropFrameOfReferencePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGraphInteropFrameOfReferencePreview> for ::windows_core::IInspectable {
    fn from(value: &SpatialGraphInteropFrameOfReferencePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialGraphInteropFrameOfReferencePreview {}
unsafe impl ::core::marker::Sync for SpatialGraphInteropFrameOfReferencePreview {}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
pub struct SpatialGraphInteropPreview;
impl SpatialGraphInteropPreview {
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CreateCoordinateSystemForNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(nodeid: Param0) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoordinateSystemForNode)(::windows_core::Interface::as_raw(this), nodeid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPosition<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(nodeid: Param0, relativeposition: Param1) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPosition)(::windows_core::Interface::as_raw(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPositionAndOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(nodeid: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CreateLocatorForNode<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(nodeid: Param0) -> ::windows_core::Result<super::SpatialLocator> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateLocatorForNode)(::windows_core::Interface::as_raw(this), nodeid.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::SpatialLocator>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn TryCreateFrameOfReference<'a, Param0: ::windows_core::IntoParam<'a, super::SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows_core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFrameOfReference)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPosition<'a, Param0: ::windows_core::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(coordinatesystem: Param0, relativeposition: Param1) -> ::windows_core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFrameOfReferenceWithPosition)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPositionAndOrientation<'a, Param0: ::windows_core::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(coordinatesystem: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows_core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFrameOfReferenceWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics2<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SpatialGraphInteropPreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
