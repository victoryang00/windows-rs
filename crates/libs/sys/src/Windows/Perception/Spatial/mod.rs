#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[repr(C)]
pub struct ISpatialAnchor {
    pub base__: ::windows_sys::core::IInspectable,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RawCoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RawCoordinateSystemAdjusted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRawCoordinateSystemAdjusted: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 86631886, data2: 7476, data3: 14082, data4: [188, 236, 234, 191, 245, 120, 168, 105] };
}
#[repr(C)]
pub struct ISpatialAnchor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAnchor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3977758984, data2: 42645, data3: 19702, data4: [146, 253, 151, 38, 59, 167, 16, 71] };
}
#[repr(C)]
pub struct ISpatialAnchorExportSufficiency {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsMinimallySufficient: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SufficiencyLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub RecommendedSufficiencyLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAnchorExportSufficiency {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2009226027, data2: 13321, data3: 16520, data4: [185, 27, 253, 253, 5, 209, 100, 143] };
}
#[repr(C)]
pub struct ISpatialAnchorExporter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub GetAnchorExportSufficiencyAsync: unsafe extern "system" fn(this: *mut *mut Self, anchor: *mut ::core::ffi::c_void, purpose: SpatialAnchorExportPurpose, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAnchorExportSufficiencyAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryExportAnchorAsync: unsafe extern "system" fn(this: *mut *mut Self, anchor: *mut ::core::ffi::c_void, purpose: SpatialAnchorExportPurpose, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryExportAnchorAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchorExporter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2586460984, data2: 9467, data3: 17001, data4: [137, 197, 136, 48, 74, 238, 242, 15] };
}
#[repr(C)]
pub struct ISpatialAnchorExporterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchorExporterStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3978627000, data2: 9333, data3: 17308, data4: [133, 255, 127, 237, 52, 31, 220, 136] };
}
#[repr(C)]
pub struct ISpatialAnchorManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchorManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2296581803, data2: 62391, data3: 16907, data4: [176, 134, 138, 128, 192, 125, 145, 13] };
}
#[repr(C)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub OldRawCoordinateSystemToNewRawCoordinateSystemTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OldRawCoordinateSystemToNewRawCoordinateSystemTransform: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2716343992, data2: 22215, data3: 12567, data4: [162, 228, 129, 224, 252, 242, 142, 0] };
}
#[repr(C)]
pub struct ISpatialAnchorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryCreateRelativeTo: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionRelativeTo: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionRelativeTo: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionAndOrientationRelativeTo: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionAndOrientationRelativeTo: usize,
}
impl ::windows_sys::core::Interface for ISpatialAnchorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2844952130, data2: 372, data3: 12572, data4: [174, 121, 14, 81, 7, 102, 159, 22] };
}
#[repr(C)]
pub struct ISpatialAnchorStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSavedAnchors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSavedAnchors: usize,
    pub TrySave: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, anchor: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialAnchorStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2965124662, data2: 18538, data3: 15536, data4: [158, 111, 18, 69, 22, 92, 77, 182] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISpatialAnchorTransferManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryImportAnchorsAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryImportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryExportAnchorsAsync: unsafe extern "system" fn(this: *mut *mut Self, anchors: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryExportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RequestAccessAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISpatialAnchorTransferManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 62650809, data2: 4824, data3: 19406, data4: [136, 53, 197, 223, 58, 192, 173, 171] };
}
#[repr(C)]
pub struct ISpatialBoundingVolume {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISpatialBoundingVolume {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4213204442, data2: 26819, data3: 13279, data4: [183, 175, 76, 120, 114, 7, 153, 156] };
}
#[repr(C)]
pub struct ISpatialBoundingVolumeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromBox: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, r#box: SpatialBoundingBox, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromOrientedBox: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, r#box: SpatialBoundingOrientedBox, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromOrientedBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromSphere: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, sphere: SpatialBoundingSphere, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromSphere: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromFrustum: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, frustum: SpatialBoundingFrustum, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromFrustum: usize,
}
impl ::windows_sys::core::Interface for ISpatialBoundingVolumeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 92836119, data2: 46049, data3: 14040, data4: [176, 23, 86, 97, 129, 165, 177, 150] };
}
#[repr(C)]
pub struct ISpatialCoordinateSystem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetTransformTo: usize,
}
impl ::windows_sys::core::Interface for ISpatialCoordinateSystem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1777060427, data2: 24739, data3: 13702, data4: [166, 83, 89, 167, 189, 103, 109, 7] };
}
#[repr(C)]
pub struct ISpatialEntity {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Anchor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
impl ::windows_sys::core::Interface for ISpatialEntity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 376301909, data2: 57835, data3: 17740, data4: [186, 8, 230, 192, 102, 141, 220, 101] };
}
#[repr(C)]
pub struct ISpatialEntityAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Entity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialEntityAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2744644763, data2: 5482, data3: 18183, data4: [172, 44, 211, 29, 87, 14, 211, 153] };
}
#[repr(C)]
pub struct ISpatialEntityFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithSpatialAnchor: unsafe extern "system" fn(this: *mut *mut Self, spatialanchor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithSpatialAnchorAndProperties: unsafe extern "system" fn(this: *mut *mut Self, spatialanchor: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithSpatialAnchorAndProperties: usize,
}
impl ::windows_sys::core::Interface for ISpatialEntityFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3790725925, data2: 13471, data3: 16933, data4: [162, 243, 75, 1, 193, 95, 224, 86] };
}
#[repr(C)]
pub struct ISpatialEntityRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Entity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialEntityRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2440304640, data2: 21357, data3: 20127, data4: [171, 246, 65, 91, 84, 68, 214, 81] };
}
#[repr(C)]
pub struct ISpatialEntityStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, entity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAsync: unsafe extern "system" fn(this: *mut *mut Self, entity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAsync: usize,
    pub CreateEntityWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialEntityStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 848791738, data2: 58643, data3: 20230, data4: [136, 157, 27, 227, 14, 207, 67, 230] };
}
#[repr(C)]
pub struct ISpatialEntityStoreStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub TryGetForRemoteSystemSession: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    TryGetForRemoteSystemSession: usize,
}
impl ::windows_sys::core::Interface for ISpatialEntityStoreStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1800091806, data2: 31824, data3: 20114, data4: [138, 98, 77, 29, 75, 124, 205, 62] };
}
#[repr(C)]
pub struct ISpatialEntityUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Entity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialEntityUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3848738662, data2: 25211, data3: 17355, data4: [164, 159, 179, 190, 109, 71, 222, 237] };
}
#[repr(C)]
pub struct ISpatialEntityWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialEntityWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialEntityWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3015204768, data2: 27998, data3: 19388, data4: [128, 93, 95, 229, 185, 186, 25, 89] };
}
#[repr(C)]
pub struct ISpatialLocation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearAcceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearAcceleration: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularVelocity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularVelocity: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularAcceleration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularAcceleration: usize,
}
impl ::windows_sys::core::Interface for ISpatialLocation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 495047325, data2: 9377, data3: 14293, data4: [143, 161, 57, 180, 249, 173, 103, 226] };
}
#[repr(C)]
pub struct ISpatialLocation2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularVelocityAxisAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularVelocityAxisAngle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularAccelerationAxisAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularAccelerationAxisAngle: usize,
}
impl ::windows_sys::core::Interface for ISpatialLocation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 293544982, data2: 14503, data3: 18968, data4: [180, 4, 171, 143, 171, 225, 215, 139] };
}
#[repr(C)]
pub struct ISpatialLocator {
    pub base__: ::windows_sys::core::IInspectable,
    pub Locatability: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialLocatability) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocatabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLocatabilityChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLocatabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionalTrackingDeactivating: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionalTrackingDeactivating: usize,
    pub TryLocateAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateAttachedFrameOfReferenceAtCurrentHeading: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: usize,
    pub CreateStationaryFrameOfReferenceAtCurrentLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut *mut Self, relativeposition: super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: usize,
}
impl ::windows_sys::core::Interface for ISpatialLocator {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4131883301, data2: 40460, data3: 15286, data4: [153, 126, 182, 78, 204, 162, 76, 244] };
}
#[repr(C)]
pub struct ISpatialLocatorAttachedFrameOfReference {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativePosition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativePosition: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOrientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOrientation: usize,
    pub AdjustHeading: unsafe extern "system" fn(this: *mut *mut Self, headingoffsetinradians: f64) -> ::windows_sys::core::HRESULT,
    pub GetStationaryCoordinateSystemAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetRelativeHeadingAtTimestamp: unsafe extern "system" fn(this: *mut *mut Self, timestamp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetRelativeHeadingAtTimestamp: usize,
}
impl ::windows_sys::core::Interface for ISpatialLocatorAttachedFrameOfReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3782692598, data2: 8015, data3: 18844, data4: [150, 37, 239, 94, 110, 215, 160, 72] };
}
#[repr(C)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Canceled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCanceled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3098034275, data2: 58356, data3: 13963, data4: [144, 97, 158, 169, 209, 214, 204, 22] };
}
#[repr(C)]
pub struct ISpatialLocatorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialLocatorStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3077452608, data2: 42946, data3: 13851, data4: [187, 130, 86, 233, 59, 137, 177, 187] };
}
#[repr(C)]
pub struct ISpatialStageFrameOfReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub MovementRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialMovementRange) -> ::windows_sys::core::HRESULT,
    pub LookDirectionRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SpatialLookDirectionRange) -> ::windows_sys::core::HRESULT,
    pub GetCoordinateSystemAtCurrentLocation: unsafe extern "system" fn(this: *mut *mut Self, locator: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMovementBounds: unsafe extern "system" fn(this: *mut *mut Self, coordinatesystem: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMovementBounds: usize,
}
impl ::windows_sys::core::Interface for ISpatialStageFrameOfReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2055877732, data2: 44301, data3: 17808, data4: [171, 134, 51, 6, 43, 103, 73, 38] };
}
#[repr(C)]
pub struct ISpatialStageFrameOfReferenceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestNewStageAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestNewStageAsync: usize,
}
impl ::windows_sys::core::Interface for ISpatialStageFrameOfReferenceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4153236557, data2: 41124, data3: 18844, data4: [141, 145, 168, 201, 101, 212, 6, 84] };
}
#[repr(C)]
pub struct ISpatialStationaryFrameOfReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISpatialStationaryFrameOfReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 165399737, data2: 48376, data3: 15999, data4: [190, 126, 126, 220, 203, 177, 120, 168] };
}
pub type SpatialAnchor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: Self = Self(0i32);
    pub const Sharing: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAnchorExportPurpose {}
impl ::core::clone::Clone for SpatialAnchorExportPurpose {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialAnchorExportSufficiency = *mut ::core::ffi::c_void;
pub type SpatialAnchorExporter = *mut ::core::ffi::c_void;
pub type SpatialAnchorRawCoordinateSystemAdjustedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialAnchorStore = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingFrustum {
    pub Near: super::super::Foundation::Numerics::Plane,
    pub Far: super::super::Foundation::Numerics::Plane,
    pub Right: super::super::Foundation::Numerics::Plane,
    pub Left: super::super::Foundation::Numerics::Plane,
    pub Top: super::super::Foundation::Numerics::Plane,
    pub Bottom: super::super::Foundation::Numerics::Plane,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingFrustum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingOrientedBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingSphere {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialBoundingVolume = *mut ::core::ffi::c_void;
pub type SpatialCoordinateSystem = *mut ::core::ffi::c_void;
pub type SpatialEntity = *mut ::core::ffi::c_void;
pub type SpatialEntityAddedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityRemovedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityStore = *mut ::core::ffi::c_void;
pub type SpatialEntityUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type SpatialEntityWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialEntityWatcherStatus {}
impl ::core::clone::Clone for SpatialEntityWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const PositionalTrackingActivating: Self = Self(2i32);
    pub const PositionalTrackingActive: Self = Self(3i32);
    pub const PositionalTrackingInhibited: Self = Self(4i32);
}
impl ::core::marker::Copy for SpatialLocatability {}
impl ::core::clone::Clone for SpatialLocatability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialLocation = *mut ::core::ffi::c_void;
pub type SpatialLocator = *mut ::core::ffi::c_void;
pub type SpatialLocatorAttachedFrameOfReference = *mut ::core::ffi::c_void;
pub type SpatialLocatorPositionalTrackingDeactivatingEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: Self = Self(0i32);
    pub const Omnidirectional: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialLookDirectionRange {}
impl ::core::clone::Clone for SpatialLookDirectionRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: Self = Self(0i32);
    pub const Bounded: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialMovementRange {}
impl ::core::clone::Clone for SpatialMovementRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Perception_Spatial\"`*"]
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialPerceptionAccessStatus {}
impl ::core::clone::Clone for SpatialPerceptionAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Perception_Spatial\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialRay {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SpatialStageFrameOfReference = *mut ::core::ffi::c_void;
pub type SpatialStationaryFrameOfReference = *mut ::core::ffi::c_void;
