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
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
pub type SpatialSurfaceInfo = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMesh = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMeshBuffer = *mut ::core::ffi::c_void;
pub type SpatialSurfaceMeshOptions = *mut ::core::ffi::c_void;
pub type SpatialSurfaceObserver = *mut ::core::ffi::c_void;
