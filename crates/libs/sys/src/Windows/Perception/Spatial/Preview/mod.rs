#[repr(C)]
pub struct ISpatialGraphInteropFrameOfReferencePreview {
    pub base__: ::windows_sys::core::IInspectable,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NodeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CoordinateSystemToNodeTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CoordinateSystemToNodeTransform: usize,
}
#[repr(C)]
pub struct ISpatialGraphInteropPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateCoordinateSystemForNode: unsafe extern "system" fn(this: *mut *mut Self, nodeid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPosition: unsafe extern "system" fn(this: *mut *mut Self, nodeid: ::windows_sys::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPositionAndOrientation: unsafe extern "system" fn(this: *mut *mut Self, nodeid: ::windows_sys::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPositionAndOrientation: usize,
    pub CreateLocatorForNode: unsafe extern "system" fn(this: *mut *mut Self, nodeid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISpatialGraphInteropPreviewStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreateFrameOfReference: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPosition: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPositionAndOrientation: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPositionAndOrientation: usize,
}
pub type SpatialGraphInteropFrameOfReferencePreview = *mut ::core::ffi::c_void;
