#[cfg(feature = "Preview")]
pub mod Preview;
#[cfg(feature = "Surfaces")]
pub mod Surfaces;
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0529e5ce_1d34_3702_bcec_eabff578a869);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RawCoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRawCoordinateSystemAdjusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchor2 {
    type Vtable = ISpatialAnchor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed17c908_a695_4cf6_92fd_97263ba71047);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExportSufficiency(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77c25b2b_3409_4088_b91b_fdfd05d1648f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExportSufficiency_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsMinimallySufficient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub RecommendedSufficiencyLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a2a4338_24fb_4269_89c5_88304aeef20f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAnchorExportSufficiencyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: ::windows_core::RawPtr, purpose: SpatialAnchorExportPurpose, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub TryExportAnchorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchor: ::windows_core::RawPtr, purpose: SpatialAnchorExportPurpose, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TryExportAnchorAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorExporterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorExporterStatics {
    type Vtable = ISpatialAnchorExporterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed2507b8_2475_439c_85ff_7fed341fdc88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorExporterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorManagerStatics {
    type Vtable = ISpatialAnchorManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88e30eab_f3b7_420b_b086_8a80c07d910d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1e81eb8_56c7_3117_a2e4_81e0fcf28e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub OldRawCoordinateSystemToNewRawCoordinateSystemTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OldRawCoordinateSystemToNewRawCoordinateSystemTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorStatics {
    type Vtable = ISpatialAnchorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9928642_0174_311c_ae79_0e5107669f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryCreateRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: ::winrt_foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionRelativeTo: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateWithPositionAndOrientationRelativeTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, position: ::winrt_foundation::Numerics::Vector3, orientation: ::winrt_foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateWithPositionAndOrientationRelativeTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAnchorStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0bc3636_486a_3cb0_9e6f_1245165c4db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSavedAnchors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSavedAnchors: usize,
    pub TrySave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, anchor: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISpatialAnchorTransferManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISpatialAnchorTransferManagerStatics {
    type Vtable = ISpatialAnchorTransferManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03bbf9b9_12d8_4bce_8835_c5df3ac0adab);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAnchorTransferManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryImportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryImportAnchorsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub TryExportAnchorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchors: ::windows_core::RawPtr, stream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated")))]
    TryExportAnchorsAsync: usize,
    #[cfg(feature = "deprecated")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolume(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb2065da_68c3_33df_b7af_4c787207999c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolume_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialBoundingVolumeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialBoundingVolumeStatics {
    type Vtable = ISpatialBoundingVolumeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05889117_b3e1_36d8_b017_566181a5b196);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialBoundingVolumeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, r#box: SpatialBoundingBox, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromOrientedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, r#box: SpatialBoundingOrientedBox, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromOrientedBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromSphere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, sphere: SpatialBoundingSphere, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromSphere: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FromFrustum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, frustum: SpatialBoundingFrustum, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FromFrustum: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialCoordinateSystem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69ebca4b_60a3_3586_a653_59a7bd676d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialCoordinateSystem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetTransformTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetTransformTo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntity(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x166de955_e1eb_454c_ba08_e6c0668ddc65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntity_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Anchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa397f49b_156a_4707_ac2c_d31d570ed399);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityFactory {
    type Vtable = ISpatialEntityFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1f1e325_349f_4225_a2f3_4b01c15fe056);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateWithSpatialAnchor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithSpatialAnchorAndProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spatialanchor: ::windows_core::RawPtr, propertyset: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithSpatialAnchorAndProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91741800_536d_4e9f_abf6_415b5444d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x329788ba_e513_4f06_889d_1be30ecf43e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStore_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entity: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateEntityWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityStoreStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityStoreStatics {
    type Vtable = ISpatialEntityStoreStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b4b389e_7c50_4e92_8a62_4d1d4b7ccd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityStoreStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "System_RemoteSystems")]
    pub TryGetForRemoteSystemSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System_RemoteSystems"))]
    TryGetForRemoteSystemSession: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5671766_627b_43cb_a49f_b3be6d47deed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Entity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialEntityWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3b85fa0_6d5e_4bbc_805d_5fe5b9ba1959);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialEntityWatcher_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialEntityWatcherStatus) -> ::windows_core::HRESULT,
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d81d29d_24a1_37d5_8fa1_39b4f9ad67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteLinearAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteLinearAcceleration: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularVelocity: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub AbsoluteAngularAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "deprecated")))]
    AbsoluteAngularAcceleration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocation2 {
    type Vtable = ISpatialLocation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x117f2416_38a7_4a18_b404_ab8fabe1d78b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularVelocityAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularVelocityAxisAngle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AbsoluteAngularAccelerationAxisAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AbsoluteAngularAccelerationAxisAngle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6478925_9e0c_3bb6_997e_b64ecca24cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocator_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Locatability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLocatability) -> ::windows_core::HRESULT,
    pub LocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveLocatabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePositionalTrackingDeactivating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TryLocateAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows_core::RawPtr, coordinatesystem: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateAttachedFrameOfReferenceAtCurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, relativeorientation: ::winrt_foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, relativeorientation: ::winrt_foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading: usize,
    pub CreateStationaryFrameOfReferenceAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, relativeorientation: ::winrt_foundation::Numerics::Quaternion, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeposition: ::winrt_foundation::Numerics::Vector3, relativeorientation: ::winrt_foundation::Numerics::Quaternion, relativeheadinginradians: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorAttachedFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1774ef6_1f4f_499c_9625_ef5e6ed7a048);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorAttachedFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RelativeOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRelativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRelativeOrientation: usize,
    pub AdjustHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headingoffsetinradians: f64) -> ::windows_core::HRESULT,
    pub GetStationaryCoordinateSystemAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryGetRelativeHeadingAtTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8a84063_e3f4_368b_9061_9ea9d1d6cc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialLocatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialLocatorStatics {
    type Vtable = ISpatialLocatorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb76e3340_a7c2_361b_bb82_56e93b89b1bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialLocatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a8a3464_ad0d_4590_ab86_33062b674926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MovementRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialMovementRange) -> ::windows_core::HRESULT,
    pub LookDirectionRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialLookDirectionRange) -> ::windows_core::HRESULT,
    pub GetCoordinateSystemAtCurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locator: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetMovementBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows_core::RawPtr, result_size__: *mut u32, result__: *mut *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetMovementBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStageFrameOfReferenceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStageFrameOfReferenceStatics {
    type Vtable = ISpatialStageFrameOfReferenceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf78d5c4d_a0a4_499c_8d91_a8c965d40654);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStageFrameOfReferenceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RequestNewStageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialStationaryFrameOfReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x09dbccb9_bcf8_3e7f_be7e_7edccbb178a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialStationaryFrameOfReference_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SpatialAnchor(::windows_core::IUnknown);
impl SpatialAnchor {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn RawCoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RawCoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn RawCoordinateSystemAdjusted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).RawCoordinateSystemAdjusted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRawCoordinateSystemAdjusted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRawCoordinateSystemAdjusted)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn RemovedByUser(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ISpatialAnchor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RemovedByUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn TryCreateRelativeTo<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows_core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionRelativeTo<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(coordinatesystem: Param0, position: Param1) -> ::windows_core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateWithPositionRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialAnchor>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateWithPositionAndOrientationRelativeTo<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(coordinatesystem: Param0, position: Param1, orientation: Param2) -> ::windows_core::Result<SpatialAnchor> {
        Self::ISpatialAnchorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateWithPositionAndOrientationRelativeTo)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), position.into_param().abi(), orientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialAnchor>(result__)
        })
    }
    pub fn ISpatialAnchorStatics<R, F: FnOnce(&ISpatialAnchorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAnchor, ISpatialAnchorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchor {}
impl ::core::fmt::Debug for SpatialAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchor;{0529e5ce-1d34-3702-bcec-eabff578a869})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchor {
    type Vtable = ISpatialAnchor_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAnchor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchor {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchor";
}
impl ::core::convert::From<SpatialAnchor> for ::windows_core::IUnknown {
    fn from(value: SpatialAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows_core::IUnknown {
    fn from(value: &SpatialAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAnchor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchor> for ::windows_core::IInspectable {
    fn from(value: SpatialAnchor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchor> for ::windows_core::IInspectable {
    fn from(value: &SpatialAnchor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAnchor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAnchor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchor {}
unsafe impl ::core::marker::Sync for SpatialAnchor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialAnchorExportPurpose {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialAnchorExportPurpose {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAnchorExportPurpose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportPurpose").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchorExportPurpose {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialAnchorExportPurpose;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialAnchorExportSufficiency(::windows_core::IUnknown);
impl SpatialAnchorExportSufficiency {
    pub fn IsMinimallySufficient(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsMinimallySufficient)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SufficiencyLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SufficiencyLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn RecommendedSufficiencyLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RecommendedSufficiencyLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExportSufficiency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExportSufficiency {}
impl ::core::fmt::Debug for SpatialAnchorExportSufficiency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportSufficiency").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchorExportSufficiency {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExportSufficiency;{77c25b2b-3409-4088-b91b-fdfd05d1648f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorExportSufficiency {
    type Vtable = ISpatialAnchorExportSufficiency_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAnchorExportSufficiency as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorExportSufficiency {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExportSufficiency";
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows_core::IUnknown {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows_core::IUnknown {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorExportSufficiency> for ::windows_core::IInspectable {
    fn from(value: SpatialAnchorExportSufficiency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExportSufficiency> for ::windows_core::IInspectable {
    fn from(value: &SpatialAnchorExportSufficiency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAnchorExportSufficiency {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExportSufficiency {}
unsafe impl ::core::marker::Sync for SpatialAnchorExportSufficiency {}
#[repr(transparent)]
pub struct SpatialAnchorExporter(::windows_core::IUnknown);
impl SpatialAnchorExporter {
    pub fn GetAnchorExportSufficiencyAsync<'a, Param0: ::windows_core::IntoParam<'a, SpatialAnchor>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialAnchorExportSufficiency>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAnchorExportSufficiencyAsync)(::windows_core::Interface::as_raw(this), anchor.into_param().abi(), purpose, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TryExportAnchorAsync<'a, Param0: ::windows_core::IntoParam<'a, SpatialAnchor>, Param2: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(&self, anchor: Param0, purpose: SpatialAnchorExportPurpose, stream: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryExportAnchorAsync)(::windows_core::Interface::as_raw(this), anchor.into_param().abi(), purpose, stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SpatialAnchorExporter> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialAnchorExporter>(result__)
        })
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorExporterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn ISpatialAnchorExporterStatics<R, F: FnOnce(&ISpatialAnchorExporterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAnchorExporter, ISpatialAnchorExporterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAnchorExporter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExporter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExporter {}
impl ::core::fmt::Debug for SpatialAnchorExporter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExporter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchorExporter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorExporter;{9a2a4338-24fb-4269-89c5-88304aeef20f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorExporter {
    type Vtable = ISpatialAnchorExporter_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAnchorExporter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorExporter {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorExporter";
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows_core::IUnknown {
    fn from(value: SpatialAnchorExporter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows_core::IUnknown {
    fn from(value: &SpatialAnchorExporter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorExporter> for ::windows_core::IInspectable {
    fn from(value: SpatialAnchorExporter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorExporter> for ::windows_core::IInspectable {
    fn from(value: &SpatialAnchorExporter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAnchorExporter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAnchorExporter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorExporter {}
unsafe impl ::core::marker::Sync for SpatialAnchorExporter {}
pub struct SpatialAnchorManager;
impl SpatialAnchorManager {
    pub fn RequestStoreAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialAnchorStore>> {
        Self::ISpatialAnchorManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestStoreAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialAnchorStore>>(result__)
        })
    }
    pub fn ISpatialAnchorManagerStatics<R, F: FnOnce(&ISpatialAnchorManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAnchorManager, ISpatialAnchorManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SpatialAnchorManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorManager";
}
#[repr(transparent)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(::windows_core::IUnknown);
impl SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Matrix4x4>::zeroed();
            (::windows_core::Interface::vtable(this).OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
impl ::core::fmt::Debug for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorRawCoordinateSystemAdjustedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs;{a1e81eb8-56c7-3117-a2e4-81e0fcf28e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    type Vtable = ISpatialAnchorRawCoordinateSystemAdjustedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAnchorRawCoordinateSystemAdjustedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorRawCoordinateSystemAdjustedEventArgs";
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorRawCoordinateSystemAdjustedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialAnchorRawCoordinateSystemAdjustedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
#[repr(transparent)]
pub struct SpatialAnchorStore(::windows_core::IUnknown);
impl SpatialAnchorStore {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSavedAnchors(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAllSavedAnchors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>>(result__)
        }
    }
    pub fn TrySave<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, SpatialAnchor>>(&self, id: Param0, anchor: Param1) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySave)(::windows_core::Interface::as_raw(this), id.into_param().abi(), anchor.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), id.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SpatialAnchorStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorStore {}
impl ::core::fmt::Debug for SpatialAnchorStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialAnchorStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialAnchorStore;{b0bc3636-486a-3cb0-9e6f-1245165c4db6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialAnchorStore {
    type Vtable = ISpatialAnchorStore_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialAnchorStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialAnchorStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorStore";
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows_core::IUnknown {
    fn from(value: SpatialAnchorStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows_core::IUnknown {
    fn from(value: &SpatialAnchorStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialAnchorStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAnchorStore> for ::windows_core::IInspectable {
    fn from(value: SpatialAnchorStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAnchorStore> for ::windows_core::IInspectable {
    fn from(value: &SpatialAnchorStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialAnchorStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialAnchorStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAnchorStore {}
unsafe impl ::core::marker::Sync for SpatialAnchorStore {}
#[cfg(feature = "deprecated")]
pub struct SpatialAnchorTransferManager;
#[cfg(feature = "deprecated")]
impl SpatialAnchorTransferManager {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryImportAnchorsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IInputStream>>(stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryImportAnchorsAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SpatialAnchor>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn TryExportAnchorsAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SpatialAnchor>>>, Param1: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IOutputStream>>(anchors: Param0, stream: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryExportAnchorsAsync)(::windows_core::Interface::as_raw(this), anchors.into_param().abi(), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        Self::ISpatialAnchorTransferManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ISpatialAnchorTransferManagerStatics<R, F: FnOnce(&ISpatialAnchorTransferManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialAnchorTransferManager, ISpatialAnchorTransferManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SpatialAnchorTransferManager {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialAnchorTransferManager";
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: ::winrt_foundation::Numerics::Vector3,
    pub Extents: ::winrt_foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingBox").field("Center", &self.Center).field("Extents", &self.Extents).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for SpatialBoundingBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for SpatialBoundingBox {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingBox>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingFrustum {
    pub Near: ::winrt_foundation::Numerics::Plane,
    pub Far: ::winrt_foundation::Numerics::Plane,
    pub Right: ::winrt_foundation::Numerics::Plane,
    pub Left: ::winrt_foundation::Numerics::Plane,
    pub Top: ::winrt_foundation::Numerics::Plane,
    pub Bottom: ::winrt_foundation::Numerics::Plane,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingFrustum {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingFrustum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingFrustum").field("Near", &self.Near).field("Far", &self.Far).field("Right", &self.Right).field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for SpatialBoundingFrustum {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for SpatialBoundingFrustum {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingFrustum;struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4);struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingFrustum {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingFrustum>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox {
    pub Center: ::winrt_foundation::Numerics::Vector3,
    pub Extents: ::winrt_foundation::Numerics::Vector3,
    pub Orientation: ::winrt_foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingOrientedBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingOrientedBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingOrientedBox").field("Center", &self.Center).field("Extents", &self.Extents).field("Orientation", &self.Orientation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for SpatialBoundingOrientedBox {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for SpatialBoundingOrientedBox {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingOrientedBox;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingOrientedBox {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingOrientedBox>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: ::winrt_foundation::Numerics::Vector3,
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
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingSphere {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingSphere").field("Center", &self.Center).field("Radius", &self.Radius).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for SpatialBoundingSphere {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for SpatialBoundingSphere {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialBoundingSphere;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialBoundingSphere>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct SpatialBoundingVolume(::windows_core::IUnknown);
impl SpatialBoundingVolume {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromBox<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, SpatialBoundingBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows_core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromBox)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromOrientedBox<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, SpatialBoundingOrientedBox>>(coordinatesystem: Param0, r#box: Param1) -> ::windows_core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromOrientedBox)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), r#box.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromSphere<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, SpatialBoundingSphere>>(coordinatesystem: Param0, sphere: Param1) -> ::windows_core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromSphere)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), sphere.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FromFrustum<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>, Param1: ::windows_core::IntoParam<'a, SpatialBoundingFrustum>>(coordinatesystem: Param0, frustum: Param1) -> ::windows_core::Result<SpatialBoundingVolume> {
        Self::ISpatialBoundingVolumeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromFrustum)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), frustum.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialBoundingVolume>(result__)
        })
    }
    pub fn ISpatialBoundingVolumeStatics<R, F: FnOnce(&ISpatialBoundingVolumeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialBoundingVolume, ISpatialBoundingVolumeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialBoundingVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialBoundingVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialBoundingVolume {}
impl ::core::fmt::Debug for SpatialBoundingVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialBoundingVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialBoundingVolume {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialBoundingVolume;{fb2065da-68c3-33df-b7af-4c787207999c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialBoundingVolume {
    type Vtable = ISpatialBoundingVolume_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialBoundingVolume as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialBoundingVolume {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialBoundingVolume";
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows_core::IUnknown {
    fn from(value: SpatialBoundingVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows_core::IUnknown {
    fn from(value: &SpatialBoundingVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialBoundingVolume> for ::windows_core::IInspectable {
    fn from(value: SpatialBoundingVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialBoundingVolume> for ::windows_core::IInspectable {
    fn from(value: &SpatialBoundingVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialBoundingVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialBoundingVolume {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialBoundingVolume {}
unsafe impl ::core::marker::Sync for SpatialBoundingVolume {}
#[repr(transparent)]
pub struct SpatialCoordinateSystem(::windows_core::IUnknown);
impl SpatialCoordinateSystem {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetTransformTo<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>>(&self, target: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Numerics::Matrix4x4>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetTransformTo)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Numerics::Matrix4x4>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialCoordinateSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialCoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialCoordinateSystem {}
impl ::core::fmt::Debug for SpatialCoordinateSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialCoordinateSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialCoordinateSystem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialCoordinateSystem;{69ebca4b-60a3-3586-a653-59a7bd676d07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialCoordinateSystem {
    type Vtable = ISpatialCoordinateSystem_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialCoordinateSystem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialCoordinateSystem {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialCoordinateSystem";
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows_core::IUnknown {
    fn from(value: SpatialCoordinateSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows_core::IUnknown {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialCoordinateSystem> for ::windows_core::IInspectable {
    fn from(value: SpatialCoordinateSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialCoordinateSystem> for ::windows_core::IInspectable {
    fn from(value: &SpatialCoordinateSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialCoordinateSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialCoordinateSystem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialCoordinateSystem {}
unsafe impl ::core::marker::Sync for SpatialCoordinateSystem {}
#[repr(transparent)]
pub struct SpatialEntity(::windows_core::IUnknown);
impl SpatialEntity {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Anchor(&self) -> ::windows_core::Result<SpatialAnchor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Anchor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialAnchor>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn CreateWithSpatialAnchor<'a, Param0: ::windows_core::IntoParam<'a, SpatialAnchor>>(spatialanchor: Param0) -> ::windows_core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSpatialAnchor)(::windows_core::Interface::as_raw(this), spatialanchor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialEntity>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithSpatialAnchorAndProperties<'a, Param0: ::windows_core::IntoParam<'a, SpatialAnchor>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::ValueSet>>(spatialanchor: Param0, propertyset: Param1) -> ::windows_core::Result<SpatialEntity> {
        Self::ISpatialEntityFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSpatialAnchorAndProperties)(::windows_core::Interface::as_raw(this), spatialanchor.into_param().abi(), propertyset.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialEntity>(result__)
        })
    }
    pub fn ISpatialEntityFactory<R, F: FnOnce(&ISpatialEntityFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialEntity, ISpatialEntityFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialEntity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntity {}
impl ::core::fmt::Debug for SpatialEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntity {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntity;{166de955-e1eb-454c-ba08-e6c0668ddc65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntity {
    type Vtable = ISpatialEntity_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntity as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntity {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntity";
}
impl ::core::convert::From<SpatialEntity> for ::windows_core::IUnknown {
    fn from(value: SpatialEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntity> for ::windows_core::IInspectable {
    fn from(value: SpatialEntity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntity> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntity {}
unsafe impl ::core::marker::Sync for SpatialEntity {}
#[repr(transparent)]
pub struct SpatialEntityAddedEventArgs(::windows_core::IUnknown);
impl SpatialEntityAddedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityAddedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityAddedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityAddedEventArgs;{a397f49b-156a-4707-ac2c-d31d570ed399})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityAddedEventArgs {
    type Vtable = ISpatialEntityAddedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntityAddedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityAddedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityAddedEventArgs";
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialEntityAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityAddedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntityAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntityAddedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityAddedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityAddedEventArgs {}
#[repr(transparent)]
pub struct SpatialEntityRemovedEventArgs(::windows_core::IUnknown);
impl SpatialEntityRemovedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityRemovedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityRemovedEventArgs;{91741800-536d-4e9f-abf6-415b5444d651})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityRemovedEventArgs {
    type Vtable = ISpatialEntityRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntityRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityRemovedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityRemovedEventArgs";
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialEntityRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntityRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntityRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityRemovedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityRemovedEventArgs {}
#[repr(transparent)]
pub struct SpatialEntityStore(::windows_core::IUnknown);
impl SpatialEntityStore {
    pub fn SaveAsync<'a, Param0: ::windows_core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), entity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveAsync<'a, Param0: ::windows_core::IntoParam<'a, SpatialEntity>>(&self, entity: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAsync)(::windows_core::Interface::as_raw(this), entity.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateEntityWatcher(&self) -> ::windows_core::Result<SpatialEntityWatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateEntityWatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialEntityWatcher>(result__)
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "System_RemoteSystems")]
    pub fn TryGetForRemoteSystemSession<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::RemoteSystems::RemoteSystemSession>>(session: Param0) -> ::windows_core::Result<SpatialEntityStore> {
        Self::ISpatialEntityStoreStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetForRemoteSystemSession)(::windows_core::Interface::as_raw(this), session.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialEntityStore>(result__)
        })
    }
    pub fn ISpatialEntityStoreStatics<R, F: FnOnce(&ISpatialEntityStoreStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialEntityStore, ISpatialEntityStoreStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialEntityStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityStore {}
impl ::core::fmt::Debug for SpatialEntityStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityStore").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityStore {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityStore;{329788ba-e513-4f06-889d-1be30ecf43e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityStore {
    type Vtable = ISpatialEntityStore_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntityStore as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityStore {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityStore";
}
impl ::core::convert::From<SpatialEntityStore> for ::windows_core::IUnknown {
    fn from(value: SpatialEntityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntityStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityStore> for ::windows_core::IInspectable {
    fn from(value: SpatialEntityStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityStore> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntityStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntityStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntityStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityStore {}
unsafe impl ::core::marker::Sync for SpatialEntityStore {}
#[repr(transparent)]
pub struct SpatialEntityUpdatedEventArgs(::windows_core::IUnknown);
impl SpatialEntityUpdatedEventArgs {
    pub fn Entity(&self) -> ::windows_core::Result<SpatialEntity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Entity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialEntity>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityUpdatedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityUpdatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs;{e5671766-627b-43cb-a49f-b3be6d47deed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityUpdatedEventArgs {
    type Vtable = ISpatialEntityUpdatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntityUpdatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityUpdatedEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityUpdatedEventArgs";
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialEntityUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityUpdatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntityUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntityUpdatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for SpatialEntityUpdatedEventArgs {}
#[repr(transparent)]
pub struct SpatialEntityWatcher(::windows_core::IUnknown);
impl SpatialEntityWatcher {
    pub fn Status(&self) -> ::windows_core::Result<SpatialEntityWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialEntityWatcherStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialEntityWatcherStatus>(result__)
        }
    }
    pub fn Added<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Updated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Removed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn EnumerationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialEntityWatcher, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SpatialEntityWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialEntityWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityWatcher {}
impl ::core::fmt::Debug for SpatialEntityWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityWatcher {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialEntityWatcher;{b3b85fa0-6d5e-4bbc-805d-5fe5b9ba1959})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialEntityWatcher {
    type Vtable = ISpatialEntityWatcher_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialEntityWatcher as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialEntityWatcher {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialEntityWatcher";
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows_core::IUnknown {
    fn from(value: SpatialEntityWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows_core::IUnknown {
    fn from(value: &SpatialEntityWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialEntityWatcher> for ::windows_core::IInspectable {
    fn from(value: SpatialEntityWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialEntityWatcher> for ::windows_core::IInspectable {
    fn from(value: &SpatialEntityWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialEntityWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialEntityWatcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialEntityWatcher {}
unsafe impl ::core::marker::Sync for SpatialEntityWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialEntityWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialEntityWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialEntityWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialEntityWatcherStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialEntityWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialLocatability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialLocatability {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialLocatability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLocatability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLocatability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SpatialLocation(::windows_core::IUnknown);
impl SpatialLocation {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearVelocity(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteLinearVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteLinearAcceleration(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteLinearAcceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularVelocity(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularVelocity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "deprecated"))]
    pub fn AbsoluteAngularAcceleration(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularAcceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularVelocityAxisAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<ISpatialLocation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).AbsoluteAngularAccelerationAxisAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocation {}
impl ::core::fmt::Debug for SpatialLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocation;{1d81d29d-24a1-37d5-8fa1-39b4f9ad67e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialLocation {
    type Vtable = ISpatialLocation_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialLocation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocation {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocation";
}
impl ::core::convert::From<SpatialLocation> for ::windows_core::IUnknown {
    fn from(value: SpatialLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows_core::IUnknown {
    fn from(value: &SpatialLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocation> for ::windows_core::IInspectable {
    fn from(value: SpatialLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocation> for ::windows_core::IInspectable {
    fn from(value: &SpatialLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialLocation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocation {}
unsafe impl ::core::marker::Sync for SpatialLocation {}
#[repr(transparent)]
pub struct SpatialLocator(::windows_core::IUnknown);
impl SpatialLocator {
    pub fn Locatability(&self) -> ::windows_core::Result<SpatialLocatability> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialLocatability>::zeroed();
            (::windows_core::Interface::vtable(this).Locatability)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialLocatability>(result__)
        }
    }
    pub fn LocatabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialLocator, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LocatabilityChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLocatabilityChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLocatabilityChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn PositionalTrackingDeactivating<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PositionalTrackingDeactivating)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePositionalTrackingDeactivating<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionalTrackingDeactivating)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    pub fn TryLocateAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::PerceptionTimestamp>, Param1: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>>(&self, timestamp: Param0, coordinatesystem: Param1) -> ::windows_core::Result<SpatialLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryLocateAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), coordinatesystem.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialLocation>(result__)
        }
    }
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeading)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows_core::Result<SpatialLocatorAttachedFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, result__.as_mut_ptr()).from_abi::<SpatialLocatorAttachedFrameOfReference>(result__)
        }
    }
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, relativeposition: Param0) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, relativeposition: Param0, relativeorientation: Param1, relativeheadinginradians: f64) -> ::windows_core::Result<SpatialStationaryFrameOfReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(::windows_core::Interface::as_raw(this), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), relativeheadinginradians, result__.as_mut_ptr()).from_abi::<SpatialStationaryFrameOfReference>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<SpatialLocator> {
        Self::ISpatialLocatorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialLocator>(result__)
        })
    }
    pub fn ISpatialLocatorStatics<R, F: FnOnce(&ISpatialLocatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialLocator, ISpatialLocatorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocator {}
impl ::core::fmt::Debug for SpatialLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLocator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocator;{f6478925-9e0c-3bb6-997e-b64ecca24cf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialLocator {
    type Vtable = ISpatialLocator_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialLocator as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocator {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocator";
}
impl ::core::convert::From<SpatialLocator> for ::windows_core::IUnknown {
    fn from(value: SpatialLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows_core::IUnknown {
    fn from(value: &SpatialLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocator> for ::windows_core::IInspectable {
    fn from(value: SpatialLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocator> for ::windows_core::IInspectable {
    fn from(value: &SpatialLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocator {}
unsafe impl ::core::marker::Sync for SpatialLocator {}
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(::windows_core::IUnknown);
impl SpatialLocatorAttachedFrameOfReference {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativePosition(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).RelativePosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativePosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativePosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RelativeOrientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetRelativeOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AdjustHeading)(::windows_core::Interface::as_raw(this), headingoffsetinradians).ok() }
    }
    pub fn GetStationaryCoordinateSystemAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStationaryCoordinateSystemAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn TryGetRelativeHeadingAtTimestamp<'a, Param0: ::windows_core::IntoParam<'a, super::PerceptionTimestamp>>(&self, timestamp: Param0) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetRelativeHeadingAtTimestamp)(::windows_core::Interface::as_raw(this), timestamp.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorAttachedFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorAttachedFrameOfReference {}
impl ::core::fmt::Debug for SpatialLocatorAttachedFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorAttachedFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLocatorAttachedFrameOfReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference;{e1774ef6-1f4f-499c-9625-ef5e6ed7a048})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialLocatorAttachedFrameOfReference {
    type Vtable = ISpatialLocatorAttachedFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialLocatorAttachedFrameOfReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocatorAttachedFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorAttachedFrameOfReference";
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocatorAttachedFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: SpatialLocatorAttachedFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorAttachedFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: &SpatialLocatorAttachedFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialLocatorAttachedFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorAttachedFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialLocatorAttachedFrameOfReference {}
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(::windows_core::IUnknown);
impl SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    pub fn Canceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanceled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanceled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
impl ::core::fmt::Debug for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorPositionalTrackingDeactivatingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs;{b8a84063-e3f4-368b-9061-9ea9d1d6cc16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    type Vtable = ISpatialLocatorPositionalTrackingDeactivatingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialLocatorPositionalTrackingDeactivatingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialLocatorPositionalTrackingDeactivatingEventArgs";
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialLocatorPositionalTrackingDeactivatingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SpatialLocatorPositionalTrackingDeactivatingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
unsafe impl ::core::marker::Sync for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialLookDirectionRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialLookDirectionRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialLookDirectionRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLookDirectionRange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialLookDirectionRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialLookDirectionRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialMovementRange {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialMovementRange {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialMovementRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialMovementRange").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialMovementRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialMovementRange;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for SpatialPerceptionAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SpatialPerceptionAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialPerceptionAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPerceptionAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialPerceptionAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Perception.Spatial.SpatialPerceptionAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: ::winrt_foundation::Numerics::Vector3,
    pub Direction: ::winrt_foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialRay {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialRay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialRay").field("Origin", &self.Origin).field("Direction", &self.Direction).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::Abi for SpatialRay {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows_core::RuntimeType for SpatialRay {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Perception.Spatial.SpatialRay;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialRay {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialRay>()) == 0 }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(::windows_core::IUnknown);
impl SpatialStageFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    pub fn MovementRange(&self) -> ::windows_core::Result<SpatialMovementRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialMovementRange>::zeroed();
            (::windows_core::Interface::vtable(this).MovementRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialMovementRange>(result__)
        }
    }
    pub fn LookDirectionRange(&self) -> ::windows_core::Result<SpatialLookDirectionRange> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SpatialLookDirectionRange>::zeroed();
            (::windows_core::Interface::vtable(this).LookDirectionRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialLookDirectionRange>(result__)
        }
    }
    pub fn GetCoordinateSystemAtCurrentLocation<'a, Param0: ::windows_core::IntoParam<'a, SpatialLocator>>(&self, locator: Param0) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCoordinateSystemAtCurrentLocation)(::windows_core::Interface::as_raw(this), locator.into_param().abi(), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetMovementBounds<'a, Param0: ::windows_core::IntoParam<'a, SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows_core::Result<::windows_core::Array<::winrt_foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<::winrt_foundation::Numerics::Vector3>>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetMovementBounds)(::windows_core::Interface::as_raw(this), coordinatesystem.into_param().abi(), ::windows_core::Array::<::winrt_foundation::Numerics::Vector3>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn Current() -> ::windows_core::Result<SpatialStageFrameOfReference> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialStageFrameOfReference>(result__)
        })
    }
    pub fn CurrentChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveCurrentChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows_core::Result<()> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentChanged)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() })
    }
    pub fn RequestNewStageAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        Self::ISpatialStageFrameOfReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestNewStageAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<SpatialStageFrameOfReference>>(result__)
        })
    }
    pub fn ISpatialStageFrameOfReferenceStatics<R, F: FnOnce(&ISpatialStageFrameOfReferenceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpatialStageFrameOfReference, ISpatialStageFrameOfReferenceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialStageFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStageFrameOfReference {}
impl ::core::fmt::Debug for SpatialStageFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStageFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialStageFrameOfReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStageFrameOfReference;{7a8a3464-ad0d-4590-ab86-33062b674926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialStageFrameOfReference {
    type Vtable = ISpatialStageFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialStageFrameOfReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialStageFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStageFrameOfReference";
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialStageFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: SpatialStageFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStageFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: &SpatialStageFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialStageFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialStageFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStageFrameOfReference {}
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(::windows_core::IUnknown);
impl SpatialStationaryFrameOfReference {
    pub fn CoordinateSystem(&self) -> ::windows_core::Result<SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialCoordinateSystem>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialStationaryFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStationaryFrameOfReference {}
impl ::core::fmt::Debug for SpatialStationaryFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStationaryFrameOfReference").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpatialStationaryFrameOfReference {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.SpatialStationaryFrameOfReference;{09dbccb9-bcf8-3e7f-be7e-7edccbb178a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpatialStationaryFrameOfReference {
    type Vtable = ISpatialStationaryFrameOfReference_Vtbl;
    const IID: ::windows_core::GUID = <ISpatialStationaryFrameOfReference as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpatialStationaryFrameOfReference {
    const NAME: &'static str = "Windows.Perception.Spatial.SpatialStationaryFrameOfReference";
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows_core::IUnknown {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialStationaryFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: SpatialStationaryFrameOfReference) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialStationaryFrameOfReference> for ::windows_core::IInspectable {
    fn from(value: &SpatialStationaryFrameOfReference) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpatialStationaryFrameOfReference {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialStationaryFrameOfReference {}
unsafe impl ::core::marker::Sync for SpatialStationaryFrameOfReference {}
