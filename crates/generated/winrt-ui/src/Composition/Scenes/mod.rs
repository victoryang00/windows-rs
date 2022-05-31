#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneBoundingBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d8ffc70_c618_4083_8251_9962593114aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneBoundingBox_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Center: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Extents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Extents: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Max: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Min: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponent {
    type Vtable = ISceneComponent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae20fc96_226c_44bd_95cb_dd5ed9ebe9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ComponentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneComponentType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponentCollection {
    type Vtable = ISceneComponentCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc483791c_5f46_45e4_b666_a3d2259f9b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneComponentFactory {
    type Vtable = ISceneComponentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fbc5574_ddd8_5889_ab5b_d8fa716e7c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ca74b7c_30df_4e07_9490_37875af1a123);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterial_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialFactory {
    type Vtable = ISceneMaterialFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67536c19_a707_5254_a495_7fdc799893b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x422a1642_1ef1_485c_97e9_ae6f95ad812f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInput_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInputFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMaterialInputFactory {
    type Vtable = ISceneMaterialInputFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa88feb74_7d0a_5e4c_a748_1015af9ca74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMesh(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMesh {
    type Vtable = ISceneMesh_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee9a1530_1155_4c0c_92bd_40020cf78347);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMesh_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub PrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrimitiveTopology: usize,
    #[cfg(feature = "winrt-graphics")]
    pub SetPrimitiveTopology: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    SetPrimitiveTopology: usize,
    #[cfg(feature = "winrt-graphics")]
    pub FillMeshAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, semantic: SceneAttributeSemantic, format: ::winrt_graphics::DirectX::DirectXPixelFormat, memory: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    FillMeshAttribute: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshMaterialAttributeMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce843171_3d43_4855_aa69_31ff988d049d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9929f7e3_6364_477e_98fe_74ed9fd4c2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Material: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Mesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UVMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshRendererComponentStatics {
    type Vtable = ISceneMeshRendererComponentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4954f37a_4459_4521_bd6e_2b38b8d711ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMeshStatics {
    type Vtable = ISceneMeshStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8412316c_7b57_473f_966b_81dc277b1751);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1d91446_799c_429e_a4e4_5da645f18e61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BaseColorInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBaseColorInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub BaseColorFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    BaseColorFactor: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetBaseColorFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetBaseColorFactor: usize,
    pub MetallicFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetMetallicFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub MetallicRoughnessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetMetallicRoughnessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RoughnessFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRoughnessFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterialStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneMetallicRoughnessMaterialStatics {
    type Vtable = ISceneMetallicRoughnessMaterialStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bddca50_6d9d_4531_8dc4_b27e3e49b7ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneModelTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc05576c2_32b1_4269_980d_b98537100ae4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModelTransform_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Orientation: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Quaternion) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetOrientation: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RotationAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RotationAxis: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetRotationAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetRotationAxis: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Scale: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetScale: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Translation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Translation: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetTranslation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNode {
    type Vtable = ISceneNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacf2c247_f307_4581_9c41_af2e29c3b016);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Components: usize,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FindFirstComponentOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneComponentType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNodeCollection {
    type Vtable = ISceneNodeCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29ada101_2dd9_4332_be63_60d2cf4269f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneNodeStatics {
    type Vtable = ISceneNodeStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x579a0faa_be9d_4210_908c_93d15feed0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneObject {
    type Vtable = ISceneObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e94249b_0f1b_49eb_a819_877d8450005b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObject_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneObjectFactory {
    type Vtable = ISceneObjectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14fe799a_33e4_52ef_956c_44229d21f2c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaab6ebbe_d680_46df_8294_b6800a9f95e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterial_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlphaCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetAlphaCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneAlphaMode) -> ::windows_core::HRESULT,
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneAlphaMode) -> ::windows_core::HRESULT,
    pub EmissiveInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEmissiveInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub EmissiveFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    EmissiveFactor: usize,
    #[cfg(feature = "winrt-foundation")]
    pub SetEmissiveFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector3) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SetEmissiveFactor: usize,
    pub IsDoubleSided: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDoubleSided: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub NormalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNormalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NormalScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetNormalScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub OcclusionInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOcclusionInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OcclusionStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetOcclusionStrength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScenePbrMaterialFactory {
    type Vtable = IScenePbrMaterialFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e3f3dfe_0b85_5727_b5be_b7d3cbac37fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1acb857_cf4f_4025_9b25_a2d1944cf507);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneRendererComponentFactory {
    type Vtable = ISceneRendererComponentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1db6ed6c_aa2c_5967_9035_56352dc69658);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9937da5c_a9ca_4cfc_b3aa_088356518742);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::CompositionBitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::CompositionBitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WrappingUMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows_core::HRESULT,
    pub SetWrappingUMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows_core::HRESULT,
    pub WrappingVMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneWrappingMode) -> ::windows_core::HRESULT,
    pub SetWrappingVMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneWrappingMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInputStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneSurfaceMaterialInputStatics {
    type Vtable = ISceneSurfaceMaterialInputStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a2394d3_6429_4589_bbcf_b84f4f3cfbfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneVisual {
    type Vtable = ISceneVisual_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e672c1e_d734_47b1_be14_3d694ffa4301);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisual_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisualStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneVisualStatics {
    type Vtable = ISceneVisualStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8347e9a_50aa_4527_8d34_de4cb8ea88b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisualStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: Self = Self(0i32);
    pub const AlphaTest: Self = Self(1i32);
    pub const Blend: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAlphaMode {}
impl ::core::clone::Clone for SceneAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneAlphaMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAlphaMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAlphaMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAlphaMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: Self = Self(0i32);
    pub const Vertex: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const TexCoord0: Self = Self(3i32);
    pub const TexCoord1: Self = Self(4i32);
    pub const Color: Self = Self(5i32);
    pub const Tangent: Self = Self(6i32);
}
impl ::core::marker::Copy for SceneAttributeSemantic {}
impl ::core::clone::Clone for SceneAttributeSemantic {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAttributeSemantic {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneAttributeSemantic {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneAttributeSemantic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAttributeSemantic").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneAttributeSemantic;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SceneBoundingBox(::windows_core::IUnknown);
impl SceneBoundingBox {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Center(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Extents(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Extents)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Max(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Max)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Min(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Min)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneBoundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneBoundingBox {}
impl ::core::fmt::Debug for SceneBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneBoundingBox").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneBoundingBox {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneBoundingBox;{5d8ffc70-c618-4083-8251-9962593114aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
    const IID: ::windows_core::GUID = <ISceneBoundingBox as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneBoundingBox";
}
impl ::core::convert::From<SceneBoundingBox> for ::windows_core::IUnknown {
    fn from(value: SceneBoundingBox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneBoundingBox> for ::windows_core::IUnknown {
    fn from(value: &SceneBoundingBox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneBoundingBox> for ::windows_core::IInspectable {
    fn from(value: SceneBoundingBox) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneBoundingBox> for ::windows_core::IInspectable {
    fn from(value: &SceneBoundingBox) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneBoundingBox> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneBoundingBox> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneBoundingBox> for SceneObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for SceneObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneBoundingBox> for super::CompositionObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for super::CompositionObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneBoundingBox {}
unsafe impl ::core::marker::Sync for SceneBoundingBox {}
#[repr(transparent)]
pub struct SceneComponent(::windows_core::IUnknown);
impl SceneComponent {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneComponentType>::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneComponentType>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneComponent {}
impl ::core::fmt::Debug for SceneComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneComponent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneComponent {
    type Vtable = ISceneComponent_Vtbl;
    const IID: ::windows_core::GUID = <ISceneComponent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponent";
}
impl ::core::convert::From<SceneComponent> for ::windows_core::IUnknown {
    fn from(value: SceneComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneComponent> for ::windows_core::IUnknown {
    fn from(value: &SceneComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneComponent> for ::windows_core::IInspectable {
    fn from(value: SceneComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneComponent> for ::windows_core::IInspectable {
    fn from(value: &SceneComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneComponent> for SceneObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for SceneObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneComponent> for super::CompositionObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for super::CompositionObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneComponent {}
unsafe impl ::core::marker::Sync for SceneComponent {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct SceneComponentCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl SceneComponentCollection {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<SceneComponent>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<SceneComponent>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<SceneComponent>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<SceneComponent>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SceneComponent>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SceneComponent>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, SceneComponent>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, SceneComponent>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, SceneComponent>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, SceneComponent>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<SceneComponent>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<SceneComponent>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for SceneComponentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for SceneComponentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for SceneComponentCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for SceneComponentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for SceneComponentCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneComponentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneComponent;{ae20fc96-226c-44bd-95cb-dd5ed9ebe9a5})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for SceneComponentCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<SceneComponent>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<SceneComponent> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneComponentCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneComponentCollection> for ::windows_core::IUnknown {
    fn from(value: SceneComponentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneComponentCollection> for ::windows_core::IUnknown {
    fn from(value: &SceneComponentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneComponentCollection> for ::windows_core::IInspectable {
    fn from(value: SceneComponentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneComponentCollection> for ::windows_core::IInspectable {
    fn from(value: &SceneComponentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneComponentCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneComponentCollection> for ::winrt_foundation::Collections::IIterable<SceneComponent> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for ::winrt_foundation::Collections::IIterable<SceneComponent> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SceneComponent>> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SceneComponent>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SceneComponent>> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SceneComponent>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<SceneComponent>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneComponentCollection> for ::winrt_foundation::Collections::IVector<SceneComponent> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneComponentCollection> for ::winrt_foundation::Collections::IVector<SceneComponent> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SceneComponent>> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SceneComponent>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SceneComponent>> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SceneComponent>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<SceneComponent>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneComponentCollection> for SceneObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneComponentCollection> for SceneObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneComponentCollection> for super::CompositionObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneComponentCollection> for super::CompositionObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for SceneComponentCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for SceneComponentCollection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: Self = Self(0i32);
}
impl ::core::marker::Copy for SceneComponentType {}
impl ::core::clone::Clone for SceneComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneComponentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneComponentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneComponentType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneComponentType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SceneMaterial(::windows_core::IUnknown);
impl SceneMaterial {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterial {}
impl ::core::fmt::Debug for SceneMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMaterial {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterial;{8ca74b7c-30df-4e07-9490-37875af1a123})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMaterial as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterial";
}
impl ::core::convert::From<SceneMaterial> for ::windows_core::IUnknown {
    fn from(value: SceneMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMaterial> for ::windows_core::IUnknown {
    fn from(value: &SceneMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMaterial> for ::windows_core::IInspectable {
    fn from(value: SceneMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMaterial> for ::windows_core::IInspectable {
    fn from(value: &SceneMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMaterial> for SceneObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for SceneObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMaterial> for super::CompositionObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for super::CompositionObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMaterial {}
unsafe impl ::core::marker::Sync for SceneMaterial {}
#[repr(transparent)]
pub struct SceneMaterialInput(::windows_core::IUnknown);
impl SceneMaterialInput {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterialInput {}
impl ::core::fmt::Debug for SceneMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterialInput").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMaterialInput {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMaterialInput;{422a1642-1ef1-485c-97e9-ae6f95ad812f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMaterialInput as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMaterialInput";
}
impl ::core::convert::From<SceneMaterialInput> for ::windows_core::IUnknown {
    fn from(value: SceneMaterialInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMaterialInput> for ::windows_core::IUnknown {
    fn from(value: &SceneMaterialInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMaterialInput> for ::windows_core::IInspectable {
    fn from(value: SceneMaterialInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMaterialInput> for ::windows_core::IInspectable {
    fn from(value: &SceneMaterialInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMaterialInput> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterialInput> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMaterialInput> for SceneObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for SceneObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMaterialInput> for super::CompositionObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for super::CompositionObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMaterialInput {}
unsafe impl ::core::marker::Sync for SceneMaterialInput {}
#[repr(transparent)]
pub struct SceneMesh(::windows_core::IUnknown);
impl SceneMesh {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Bounds(&self) -> ::windows_core::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneBoundingBox>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrimitiveTopology(&self) -> ::windows_core::Result<::winrt_graphics::DirectX::DirectXPrimitiveTopology> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DirectX::DirectXPrimitiveTopology>::zeroed();
            (::windows_core::Interface::vtable(this).PrimitiveTopology)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DirectX::DirectXPrimitiveTopology>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn SetPrimitiveTopology(&self, value: ::winrt_graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrimitiveTopology)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn FillMeshAttribute<'a, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::MemoryBuffer>>(&self, semantic: SceneAttributeSemantic, format: ::winrt_graphics::DirectX::DirectXPixelFormat, memory: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).FillMeshAttribute)(::windows_core::Interface::as_raw(this), semantic, format, memory.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneMesh> {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneMesh>(result__)
        })
    }
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneMesh, ISceneMeshStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMesh {}
impl ::core::fmt::Debug for SceneMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMesh {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMesh;{ee9a1530-1155-4c0c-92bd-40020cf78347})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMesh {
    type Vtable = ISceneMesh_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMesh as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMesh {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMesh";
}
impl ::core::convert::From<SceneMesh> for ::windows_core::IUnknown {
    fn from(value: SceneMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMesh> for ::windows_core::IUnknown {
    fn from(value: &SceneMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMesh> for ::windows_core::IInspectable {
    fn from(value: SceneMesh) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMesh> for ::windows_core::IInspectable {
    fn from(value: &SceneMesh) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMesh> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMesh) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMesh> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMesh) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMesh> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMesh) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMesh> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMesh) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMesh> for SceneObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for SceneObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMesh> for super::CompositionObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for super::CompositionObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMesh {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMesh {}
unsafe impl ::core::marker::Sync for SceneMesh {}
#[repr(transparent)]
pub struct SceneMeshMaterialAttributeMap(::windows_core::IUnknown);
impl SceneMeshMaterialAttributeMap {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<SceneAttributeSemantic> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneAttributeSemantic>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneAttributeSemantic>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SceneAttributeSemantic>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, SceneAttributeSemantic>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0, value: SceneAttributeSemantic) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMeshMaterialAttributeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshMaterialAttributeMap {}
impl ::core::fmt::Debug for SceneMeshMaterialAttributeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshMaterialAttributeMap").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap;{ce843171-3d43-4855-aa69-31ff988d049d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMeshMaterialAttributeMap as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for ::windows_core::IUnknown {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for ::windows_core::IUnknown {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for ::windows_core::IInspectable {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for ::windows_core::IInspectable {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, SceneAttributeSemantic>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, SceneAttributeSemantic>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMeshMaterialAttributeMap {}
unsafe impl ::core::marker::Sync for SceneMeshMaterialAttributeMap {}
#[repr(transparent)]
pub struct SceneMeshRendererComponent(::windows_core::IUnknown);
impl SceneMeshRendererComponent {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = &::windows_core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneComponentType>::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneComponentType>(result__)
        }
    }
    pub fn Material(&self) -> ::windows_core::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Material)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterial>(result__)
        }
    }
    pub fn SetMaterial<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterial>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaterial)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Mesh(&self) -> ::windows_core::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Mesh)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMesh>(result__)
        }
    }
    pub fn SetMesh<'a, Param0: ::windows_core::IntoParam<'a, SceneMesh>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMesh)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn UVMappings(&self) -> ::windows_core::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UVMappings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMeshMaterialAttributeMap>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneMeshRendererComponent> {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneMeshRendererComponent>(result__)
        })
    }
    pub fn ISceneMeshRendererComponentStatics<R, F: FnOnce(&ISceneMeshRendererComponentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneMeshRendererComponent, ISceneMeshRendererComponentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneMeshRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMeshRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshRendererComponent {}
impl ::core::fmt::Debug for SceneMeshRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshRendererComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMeshRendererComponent;{9929f7e3-6364-477e-98fe-74ed9fd4c2de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMeshRendererComponent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMeshRendererComponent";
}
impl ::core::convert::From<SceneMeshRendererComponent> for ::windows_core::IUnknown {
    fn from(value: SceneMeshRendererComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for ::windows_core::IUnknown {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for ::windows_core::IInspectable {
    fn from(value: SceneMeshRendererComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for ::windows_core::IInspectable {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneRendererComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneRendererComponent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneRendererComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneRendererComponent> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneRendererComponent>::into(self))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneComponent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneComponent> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneComponent>::into(self))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMeshRendererComponent {}
unsafe impl ::core::marker::Sync for SceneMeshRendererComponent {}
#[repr(transparent)]
pub struct SceneMetallicRoughnessMaterial(::windows_core::IUnknown);
impl SceneMetallicRoughnessMaterial {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn BaseColorInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BaseColorInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetBaseColorInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseColorInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn BaseColorFactor(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector4>::zeroed();
            (::windows_core::Interface::vtable(this).BaseColorFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector4>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetBaseColorFactor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector4>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBaseColorFactor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MetallicFactor(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MetallicFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetMetallicFactor(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMetallicFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MetallicRoughnessInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MetallicRoughnessInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetMetallicRoughnessInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMetallicRoughnessInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RoughnessFactor(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RoughnessFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRoughnessFactor(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoughnessFactor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneMetallicRoughnessMaterial> {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneMetallicRoughnessMaterial>(result__)
        })
    }
    pub fn AlphaCutoff(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).AlphaCutoff)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> ::windows_core::Result<SceneAlphaMode> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).AlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneAlphaMode>(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetEmissiveInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn EmissiveFactor(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetEmissiveFactor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveFactor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDoubleSided(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleSided)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDoubleSided)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NormalInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetNormalInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NormalScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).NormalScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetOcclusionInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionStrength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionStrength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ISceneMetallicRoughnessMaterialStatics<R, F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneMetallicRoughnessMaterial, ISceneMetallicRoughnessMaterialStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMetallicRoughnessMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMetallicRoughnessMaterial {}
impl ::core::fmt::Debug for SceneMetallicRoughnessMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMetallicRoughnessMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial;{c1d91446-799c-429e-a4e4-5da645f18e61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
    const IID: ::windows_core::GUID = <ISceneMetallicRoughnessMaterial as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ::windows_core::IUnknown {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ::windows_core::IUnknown {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ::windows_core::IInspectable {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ::windows_core::IInspectable {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ScenePbrMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ScenePbrMaterial> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ScenePbrMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ScenePbrMaterial> {
        ::windows_core::Param::Owned(::core::convert::Into::<ScenePbrMaterial>::into(self))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterial> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterial> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneMaterial>::into(self))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneMetallicRoughnessMaterial {}
unsafe impl ::core::marker::Sync for SceneMetallicRoughnessMaterial {}
#[repr(transparent)]
pub struct SceneModelTransform(::windows_core::IUnknown);
impl SceneModelTransform {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Orientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotationAxis(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAxis)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetRotationAxis<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAxis)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Scale(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetScale<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScale)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Translation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Translation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetTranslation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTranslation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SceneModelTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneModelTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneModelTransform {}
impl ::core::fmt::Debug for SceneModelTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneModelTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneModelTransform {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneModelTransform;{c05576c2-32b1-4269-980d-b98537100ae4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
    const IID: ::windows_core::GUID = <ISceneModelTransform as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneModelTransform";
}
impl ::core::convert::From<SceneModelTransform> for ::windows_core::IUnknown {
    fn from(value: SceneModelTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneModelTransform> for ::windows_core::IUnknown {
    fn from(value: &SceneModelTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneModelTransform> for ::windows_core::IInspectable {
    fn from(value: SceneModelTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneModelTransform> for ::windows_core::IInspectable {
    fn from(value: &SceneModelTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneModelTransform) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneModelTransform> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneModelTransform) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneModelTransform> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionTransform {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionTransform {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionTransform> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionTransform> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionTransform> for &SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionTransform> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionTransform>::into(self))
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionObject {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionObject {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneModelTransform {}
unsafe impl ::core::marker::Sync for SceneModelTransform {}
#[repr(transparent)]
pub struct SceneNode(::windows_core::IUnknown);
impl SceneNode {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneNodeCollection>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Components(&self) -> ::windows_core::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Components)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneComponentCollection>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneNode>(result__)
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneModelTransform>(result__)
        }
    }
    pub fn FindFirstComponentOfType(&self, value: SceneComponentType) -> ::windows_core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindFirstComponentOfType)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<SceneComponent>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneNode> {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneNode>(result__)
        })
    }
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneNode, ISceneNodeStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneNode {}
impl ::core::fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneNode {
    type Vtable = ISceneNode_Vtbl;
    const IID: ::windows_core::GUID = <ISceneNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneNode {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNode";
}
impl ::core::convert::From<SceneNode> for ::windows_core::IUnknown {
    fn from(value: SceneNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneNode> for ::windows_core::IUnknown {
    fn from(value: &SceneNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneNode> for ::windows_core::IInspectable {
    fn from(value: SceneNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneNode> for ::windows_core::IInspectable {
    fn from(value: &SceneNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneNode> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNode> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNode> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneNode> for SceneObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNode> for SceneObject {
    fn from(value: &SceneNode) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneNode> for super::CompositionObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNode> for super::CompositionObject {
    fn from(value: &SceneNode) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneNode {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneNode {}
unsafe impl ::core::marker::Sync for SceneNode {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct SceneNodeCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl SceneNodeCollection {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<SceneNode>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<SceneNode>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<SceneNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<SceneNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SceneNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SceneNode>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, SceneNode>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, SceneNode>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, SceneNode>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, SceneNode>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<SceneNode>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<SceneNode>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for SceneNodeCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for SceneNodeCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for SceneNodeCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for SceneNodeCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNodeCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for SceneNodeCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneNodeCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Composition.Scenes.SceneNode;{acf2c247-f307-4581-9c41-af2e29c3b016})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for SceneNodeCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<SceneNode>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<SceneNode> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneNodeCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneNodeCollection> for ::windows_core::IUnknown {
    fn from(value: SceneNodeCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneNodeCollection> for ::windows_core::IUnknown {
    fn from(value: &SceneNodeCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneNodeCollection> for ::windows_core::IInspectable {
    fn from(value: SceneNodeCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneNodeCollection> for ::windows_core::IInspectable {
    fn from(value: &SceneNodeCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneNodeCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneNodeCollection> for ::winrt_foundation::Collections::IIterable<SceneNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for ::winrt_foundation::Collections::IIterable<SceneNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SceneNode>> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SceneNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<SceneNode>> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<SceneNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<SceneNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<SceneNodeCollection> for ::winrt_foundation::Collections::IVector<SceneNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&SceneNodeCollection> for ::winrt_foundation::Collections::IVector<SceneNode> {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SceneNode>> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SceneNode>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<SceneNode>> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<SceneNode>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<SceneNode>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneNodeCollection> for SceneObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneNodeCollection> for SceneObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<SceneNodeCollection> for super::CompositionObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&SceneNodeCollection> for super::CompositionObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for SceneNodeCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for SceneNodeCollection {}
#[repr(transparent)]
pub struct SceneObject(::windows_core::IUnknown);
impl SceneObject {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneObject {}
impl ::core::fmt::Debug for SceneObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneObject {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneObject;{1e94249b-0f1b-49eb-a819-877d8450005b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneObject {
    type Vtable = ISceneObject_Vtbl;
    const IID: ::windows_core::GUID = <ISceneObject as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneObject {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneObject";
}
impl ::core::convert::From<SceneObject> for ::windows_core::IUnknown {
    fn from(value: SceneObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneObject> for ::windows_core::IUnknown {
    fn from(value: &SceneObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneObject> for ::windows_core::IInspectable {
    fn from(value: SceneObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneObject> for ::windows_core::IInspectable {
    fn from(value: &SceneObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneObject> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneObject> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneObject> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneObject> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneObject> for super::CompositionObject {
    fn from(value: SceneObject) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneObject> for super::CompositionObject {
    fn from(value: &SceneObject) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneObject {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneObject {}
unsafe impl ::core::marker::Sync for SceneObject {}
#[repr(transparent)]
pub struct ScenePbrMaterial(::windows_core::IUnknown);
impl ScenePbrMaterial {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn AlphaCutoff(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).AlphaCutoff)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaCutoff)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlphaMode(&self) -> ::windows_core::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneAlphaMode>::zeroed();
            (::windows_core::Interface::vtable(this).AlphaMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneAlphaMode>(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlphaMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EmissiveInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetEmissiveInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn EmissiveFactor(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).EmissiveFactor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetEmissiveFactor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmissiveFactor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsDoubleSided(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleSided)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDoubleSided)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NormalInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NormalInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetNormalInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn NormalScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).NormalScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalScale)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OcclusionInput(&self) -> ::windows_core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetOcclusionInput<'a, Param0: ::windows_core::IntoParam<'a, SceneMaterialInput>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OcclusionStrength(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).OcclusionStrength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOcclusionStrength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ScenePbrMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScenePbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScenePbrMaterial {}
impl ::core::fmt::Debug for ScenePbrMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScenePbrMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScenePbrMaterial {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.ScenePbrMaterial;{aab6ebbe-d680-46df-8294-b6800a9f95e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
    const IID: ::windows_core::GUID = <IScenePbrMaterial as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.ScenePbrMaterial";
}
impl ::core::convert::From<ScenePbrMaterial> for ::windows_core::IUnknown {
    fn from(value: ScenePbrMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for ::windows_core::IUnknown {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScenePbrMaterial> for ::windows_core::IInspectable {
    fn from(value: ScenePbrMaterial) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for ::windows_core::IInspectable {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ScenePbrMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScenePbrMaterial> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneMaterial {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneMaterial {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterial> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterial> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterial> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterial> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneMaterial>::into(self))
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<ScenePbrMaterial> for super::CompositionObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for super::CompositionObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ScenePbrMaterial {}
unsafe impl ::core::marker::Sync for ScenePbrMaterial {}
#[repr(transparent)]
pub struct SceneRendererComponent(::windows_core::IUnknown);
impl SceneRendererComponent {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn ComponentType(&self) -> ::windows_core::Result<SceneComponentType> {
        let this = &::windows_core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneComponentType>::zeroed();
            (::windows_core::Interface::vtable(this).ComponentType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneComponentType>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneRendererComponent {}
impl ::core::fmt::Debug for SceneRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneRendererComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneRendererComponent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneRendererComponent;{f1acb857-cf4f-4025-9b25-a2d1944cf507})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
    const IID: ::windows_core::GUID = <ISceneRendererComponent as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneRendererComponent";
}
impl ::core::convert::From<SceneRendererComponent> for ::windows_core::IUnknown {
    fn from(value: SceneRendererComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneRendererComponent> for ::windows_core::IUnknown {
    fn from(value: &SceneRendererComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneRendererComponent> for ::windows_core::IInspectable {
    fn from(value: SceneRendererComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneRendererComponent> for ::windows_core::IInspectable {
    fn from(value: &SceneRendererComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneRendererComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneRendererComponent> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneComponent {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneComponent {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneComponent> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneComponent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneComponent> for &SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneComponent> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneComponent>::into(self))
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneRendererComponent> for super::CompositionObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for super::CompositionObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneRendererComponent {}
unsafe impl ::core::marker::Sync for SceneRendererComponent {}
#[repr(transparent)]
pub struct SceneSurfaceMaterialInput(::windows_core::IUnknown);
impl SceneSurfaceMaterialInput {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn BitmapInterpolationMode(&self) -> ::windows_core::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::CompositionBitmapInterpolationMode>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapInterpolationMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionBitmapInterpolationMode>(result__)
        }
    }
    pub fn SetBitmapInterpolationMode(&self, value: super::CompositionBitmapInterpolationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitmapInterpolationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Surface(&self) -> ::windows_core::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Surface)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ICompositionSurface>(result__)
        }
    }
    pub fn SetSurface<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionSurface>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSurface)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn WrappingUMode(&self) -> ::windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneWrappingMode>::zeroed();
            (::windows_core::Interface::vtable(this).WrappingUMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneWrappingMode>(result__)
        }
    }
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWrappingUMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WrappingVMode(&self) -> ::windows_core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SceneWrappingMode>::zeroed();
            (::windows_core::Interface::vtable(this).WrappingVMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneWrappingMode>(result__)
        }
    }
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWrappingVMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneSurfaceMaterialInput> {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneSurfaceMaterialInput>(result__)
        })
    }
    pub fn ISceneSurfaceMaterialInputStatics<R, F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneSurfaceMaterialInput, ISceneSurfaceMaterialInputStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneSurfaceMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneSurfaceMaterialInput {}
impl ::core::fmt::Debug for SceneSurfaceMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneSurfaceMaterialInput").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput;{9937da5c-a9ca-4cfc-b3aa-088356518742})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
    const IID: ::windows_core::GUID = <ISceneSurfaceMaterialInput as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for ::windows_core::IUnknown {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for ::windows_core::IUnknown {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for ::windows_core::IInspectable {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for ::windows_core::IInspectable {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterialInput> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterialInput> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneMaterialInput> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneMaterialInput> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneMaterialInput>::into(self))
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, SceneObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, SceneObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<SceneObject>::into(self))
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneSurfaceMaterialInput {}
unsafe impl ::core::marker::Sync for SceneSurfaceMaterialInput {}
#[repr(transparent)]
pub struct SceneVisual(::windows_core::IUnknown);
impl SceneVisual {
    pub fn PopulatePropertyInfo<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Compositor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    pub fn StopAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimation)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi()).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Comment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetComment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImplicitAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<'a, Param0: ::windows_core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetImplicitAnimations)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StartAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StopAnimationGroup<'a, Param0: ::windows_core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StopAnimationGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-system")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<::winrt_system::DispatcherQueue> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, propertyname: Param0) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAnimationController)(::windows_core::Interface::as_raw(this), propertyname.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Children(&self) -> ::windows_core::Result<super::VisualCollection> {
        let this = &::windows_core::Interface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::VisualCollection>(result__)
        }
    }
    pub fn Root(&self) -> ::windows_core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Root)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SceneNode>(result__)
        }
    }
    pub fn SetRoot<'a, Param0: ::windows_core::IntoParam<'a, SceneNode>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<SceneVisual> {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<SceneVisual>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AnchorPoint(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector2> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).AnchorPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAnchorPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAnchorPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BackfaceVisibility(&self) -> ::windows_core::Result<super::CompositionBackfaceVisibility> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::CompositionBackfaceVisibility>::zeroed();
            (::windows_core::Interface::vtable(this).BackfaceVisibility)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionBackfaceVisibility>(result__)
        }
    }
    pub fn SetBackfaceVisibility(&self, value: super::CompositionBackfaceVisibility) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBackfaceVisibility)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderMode(&self) -> ::windows_core::Result<super::CompositionBorderMode> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::CompositionBorderMode>::zeroed();
            (::windows_core::Interface::vtable(this).BorderMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionBorderMode>(result__)
        }
    }
    pub fn SetBorderMode(&self, value: super::CompositionBorderMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBorderMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn CenterPoint(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).CenterPoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetCenterPoint<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCenterPoint)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Clip(&self) -> ::windows_core::Result<super::CompositionClip> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Clip)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionClip>(result__)
        }
    }
    pub fn SetClip<'a, Param0: ::windows_core::IntoParam<'a, super::CompositionClip>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetClip)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CompositeMode(&self) -> ::windows_core::Result<super::CompositionCompositeMode> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::CompositionCompositeMode>::zeroed();
            (::windows_core::Interface::vtable(this).CompositeMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionCompositeMode>(result__)
        }
    }
    pub fn SetCompositeMode(&self, value: super::CompositionCompositeMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompositeMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Offset(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetOffset<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOffset)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Opacity(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).Opacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Orientation(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Quaternion> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Quaternion>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetOrientation<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Parent(&self) -> ::windows_core::Result<super::ContainerVisual> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ContainerVisual>(result__)
        }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegrees)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotationAxis(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).RotationAxis)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetRotationAxis<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRotationAxis)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Scale(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetScale<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetScale)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector2> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetSize<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TransformMatrix(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Matrix4x4> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Matrix4x4>::zeroed();
            (::windows_core::Interface::vtable(this).TransformMatrix)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetTransformMatrix<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Matrix4x4>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransformMatrix)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ParentForTransform(&self) -> ::windows_core::Result<super::Visual> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ParentForTransform)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Visual>(result__)
        }
    }
    pub fn SetParentForTransform<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParentForTransform)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RelativeOffsetAdjustment(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector3> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector3>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeOffsetAdjustment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetRelativeOffsetAdjustment<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeOffsetAdjustment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RelativeSizeAdjustment(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector2> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector2>::zeroed();
            (::windows_core::Interface::vtable(this).RelativeSizeAdjustment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector2>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetRelativeSizeAdjustment<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeSizeAdjustment)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsHitTestVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHitTestVisible)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsHitTestVisible)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPixelSnappingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPixelSnappingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IVisual4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPixelSnappingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ISceneVisualStatics<R, F: FnOnce(&ISceneVisualStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SceneVisual, ISceneVisualStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SceneVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneVisual {}
impl ::core::fmt::Debug for SceneVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneVisual {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Scenes.SceneVisual;{8e672c1e-d734-47b1-be14-3d694ffa4301})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SceneVisual {
    type Vtable = ISceneVisual_Vtbl;
    const IID: ::windows_core::GUID = <ISceneVisual as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneVisual {
    const NAME: &'static str = "Windows.UI.Composition.Scenes.SceneVisual";
}
impl ::core::convert::From<SceneVisual> for ::windows_core::IUnknown {
    fn from(value: SceneVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneVisual> for ::windows_core::IUnknown {
    fn from(value: &SceneVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneVisual> for ::windows_core::IInspectable {
    fn from(value: SceneVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneVisual> for ::windows_core::IInspectable {
    fn from(value: &SceneVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SceneVisual> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneVisual) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneVisual> for super::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneVisual) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IAnimationObject> for &SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneVisual> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: SceneVisual) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneVisual> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &SceneVisual) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SceneVisual> for super::ContainerVisual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::ContainerVisual {
    fn from(value: &SceneVisual) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContainerVisual> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContainerVisual> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContainerVisual> for &SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContainerVisual> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContainerVisual>::into(self))
    }
}
impl ::core::convert::From<SceneVisual> for super::Visual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::Visual {
    fn from(value: &SceneVisual) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Visual> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::Visual> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Visual> for &SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::Visual> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Visual>::into(self))
    }
}
impl ::core::convert::From<SceneVisual> for super::CompositionObject {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::CompositionObject {
    fn from(value: &SceneVisual) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CompositionObject> for &SceneVisual {
    fn into_param(self) -> ::windows_core::Param<'a, super::CompositionObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SceneVisual {}
unsafe impl ::core::marker::Sync for SceneVisual {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: Self = Self(0i32);
    pub const MirroredRepeat: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneWrappingMode {}
impl ::core::clone::Clone for SceneWrappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneWrappingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SceneWrappingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneWrappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneWrappingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SceneWrappingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Scenes.SceneWrappingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
