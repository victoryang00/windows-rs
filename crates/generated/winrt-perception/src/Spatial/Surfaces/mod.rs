#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TryGetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryGetBounds: usize,
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SurfaceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub VertexPositionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-storage")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-storage"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub VertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "winrt-graphics")]
    pub TriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "winrt-graphics")]
    pub VertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceMeshOptionsStatics {
    type Vtable = ISpatialSurfaceMeshOptionsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b340abf_9781_4505_8935_013575caae5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-graphics")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-graphics")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-graphics")))]
    SupportedVertexNormalFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetObservedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SetBoundingVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetBoundingVolumes: usize,
    pub ObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserverStatics {
    type Vtable = ISpatialSurfaceObserverStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x165951ed_2108_4168_9175_87e027bc9285);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialSurfaceObserverStatics2 {
    type Vtable = ISpatialSurfaceObserverStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f534261_c55d_4e6b_a895_a19de69a42e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SpatialSurfaceInfo(::windows_core::IUnknown);
impl SpatialSurfaceInfo {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn UpdateTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryGetBounds<'a, Param0: ::windows_core::IntoParam<'a, super::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<super::SpatialBoundingOrientedBox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetBounds)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::SpatialBoundingOrientedBox>>(result__)
        }
    }
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLatestMeshAsync)(::windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
    pub fn TryComputeLatestMeshWithOptionsAsync<'a, Param1: ::windows_core::IntoParam<'a, SpatialSurfaceMeshOptions>>(&self, maxtrianglespercubicmeter: f64, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeLatestMeshWithOptionsAsync)(::windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceInfo {}
impl ::core::fmt::Debug for SpatialSurfaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo;{f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialSurfaceInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows_core::IUnknown {
    fn from(value: SpatialSurfaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows_core::IUnknown {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceInfo> for ::windows_core::IInspectable {
    fn from(value: SpatialSurfaceInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceInfo> for ::windows_core::IInspectable {
    fn from(value: &SpatialSurfaceInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialSurfaceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialSurfaceInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceInfo {}
unsafe impl ::core::marker::Sync for SpatialSurfaceInfo {}
#[repr(transparent)]
pub struct SpatialSurfaceMesh(::windows_core::IUnknown);
impl SpatialSurfaceMesh {
    pub fn SurfaceInfo(&self) -> ::windows_core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SurfaceInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceInfo>(result__)
        }
    }
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    pub fn TriangleIndices(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TriangleIndices)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    pub fn VertexPositions(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn VertexPositionScale(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositionScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn VertexNormals(&self) -> ::windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VertexNormals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMesh {}
impl ::core::fmt::Debug for SpatialSurfaceMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh;{108f57d9-df0d-3950-a0fd-f972c77c27b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialSurfaceMesh as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows_core::IUnknown {
    fn from(value: SpatialSurfaceMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows_core::IUnknown {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMesh> for ::windows_core::IInspectable {
    fn from(value: SpatialSurfaceMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMesh> for ::windows_core::IInspectable {
    fn from(value: &SpatialSurfaceMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialSurfaceMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialSurfaceMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMesh {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMesh {}
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(::windows_core::IUnknown);
impl SpatialSurfaceMeshBuffer {
    #[cfg(feature = "winrt-graphics")]
    pub fn Format(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    pub fn Stride(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Stride)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ElementCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ElementCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-storage")]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshBuffer {}
impl ::core::fmt::Debug for SpatialSurfaceMeshBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer;{93cf59e0-871f-33f8-98b2-03d101458f6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialSurfaceMeshBuffer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows_core::IUnknown {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows_core::IUnknown {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMeshBuffer> for ::windows_core::IInspectable {
    fn from(value: SpatialSurfaceMeshBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshBuffer> for ::windows_core::IInspectable {
    fn from(value: &SpatialSurfaceMeshBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialSurfaceMeshBuffer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshBuffer {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshBuffer {}
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(::windows_core::IUnknown);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialSurfaceMeshOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn VertexPositionFormat(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).VertexPositionFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetVertexPositionFormat(&self, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVertexPositionFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn TriangleIndexFormat(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).TriangleIndexFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetTriangleIndexFormat(&self, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTriangleIndexFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn VertexNormalFormat(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DirectX::DirectXPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).VertexNormalFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetVertexNormalFormat(&self, value: ::winrt_graphics::DirectX::DirectXPixelFormat) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVertexNormalFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeVertexNormals(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IncludeVertexNormals)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeVertexNormals(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeVertexNormals)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub fn SupportedVertexPositionFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVertexPositionFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub fn SupportedTriangleIndexFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTriangleIndexFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-graphics"))]
    pub fn SupportedVertexNormalFormats() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedVertexNormalFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshOptions {}
impl ::core::fmt::Debug for SpatialSurfaceMeshOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions;{d2759f89-3572-3d2d-a10d-5fee9394aa37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialSurfaceMeshOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows_core::IUnknown {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows_core::IUnknown {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceMeshOptions> for ::windows_core::IInspectable {
    fn from(value: SpatialSurfaceMeshOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceMeshOptions> for ::windows_core::IInspectable {
    fn from(value: &SpatialSurfaceMeshOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialSurfaceMeshOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceMeshOptions {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshOptions {}
#[repr(transparent)]
pub struct SpatialSurfaceObserver(::windows_core::IUnknown);
impl SpatialSurfaceObserver {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialSurfaceObserver, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetObservedSurfaces(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObservedSurfaces)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::GUID, SpatialSurfaceInfo>>(result__)
        }
    }
    pub fn SetBoundingVolume<'a, Param0: ::windows_core::IntoParam<'a, super::SpatialBoundingVolume>>(&self, bounds: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundingVolume)(::windows_core::Interface::as_raw(this), bounds.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetBoundingVolumes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::SpatialBoundingVolume>>>(&self, bounds: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundingVolumes)(::windows_core::Interface::as_raw(this), bounds.into_param().abi()).ok() }
    }
    pub fn ObservedSurfacesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ObservedSurfacesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveObservedSurfacesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveObservedSurfacesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialSurfaceObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceObserver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceObserver {}
impl ::core::fmt::Debug for SpatialSurfaceObserver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceObserver").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver;{10b69819-ddca-3483-ac3a-748fe8c86df5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialSurfaceObserver as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows_core::IUnknown {
    fn from(value: SpatialSurfaceObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows_core::IUnknown {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialSurfaceObserver> for ::windows_core::IInspectable {
    fn from(value: SpatialSurfaceObserver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialSurfaceObserver> for ::windows_core::IInspectable {
    fn from(value: &SpatialSurfaceObserver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialSurfaceObserver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialSurfaceObserver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialSurfaceObserver {}
unsafe impl ::core::marker::Sync for SpatialSurfaceObserver {}
