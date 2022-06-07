#[repr(C)]
pub struct ISpatialSurfaceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetBounds: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(this: *mut *mut Self, maxtrianglespercubicmeter: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, maxtrianglespercubicmeter: f64, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshWithOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4176079847, data2: 14775, data3: 14690, data4: [187, 3, 87, 245, 110, 31, 176, 161] };
}
#[repr(C)]
pub struct ISpatialSurfaceMesh {
    pub base__: ::windows_sys::core::IInspectable,
    pub SurfaceInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub VertexPositionScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceMesh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 277829593, data2: 57101, data3: 14672, data4: [160, 253, 249, 114, 199, 124, 39, 180] };
}
#[repr(C)]
pub struct ISpatialSurfaceMeshBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceMeshBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479839712, data2: 34591, data3: 13304, data4: [152, 178, 3, 209, 1, 69, 143, 111] };
}
#[repr(C)]
pub struct ISpatialSurfaceMeshOptions {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexPositionFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub TriangleIndexFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexNormalFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceMeshOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3530923913, data2: 13682, data3: 15661, data4: [161, 13, 95, 238, 147, 148, 170, 55] };
}
#[repr(C)]
pub struct ISpatialSurfaceMeshOptionsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexNormalFormats: usize,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceMeshOptionsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2603879103, data2: 38785, data3: 17669, data4: [137, 53, 1, 53, 117, 202, 174, 94] };
}
#[repr(C)]
pub struct ISpatialSurfaceObserver {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObservedSurfaces: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetBoundingVolumes: unsafe extern "system" fn(this: *mut *mut Self, bounds: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetBoundingVolumes: usize,
    #[cfg(feature = "Foundation")]
    pub ObservedSurfacesChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ObservedSurfacesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveObservedSurfacesChanged: usize,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceObserver {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 280401945, data2: 56778, data3: 13443, data4: [172, 58, 116, 143, 232, 200, 109, 245] };
}
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceObserverStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 374952429, data2: 8456, data3: 16744, data4: [145, 117, 135, 224, 39, 188, 146, 133] };
}
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialSurfaceObserverStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 257114721, data2: 50525, data3: 20075, data4: [168, 149, 161, 157, 230, 154, 66, 227] };
}
pub type SpatialSurfaceInfo = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMesh = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMeshBuffer = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMeshOptions = *mut ::core::ffi::c_void;
pub type SpatialSurfaceObserver = *mut ::core::ffi::c_void;
