#[repr(C)]
pub struct ISceneBoundingBox {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Center: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Extents: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Extents: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Max: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Max: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Min: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Min: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Size: usize,
}
#[repr(C)]
pub struct ISceneComponent {
    pub base__: ::windows_sys::core::IInspectable,
    pub ComponentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneComponentType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneComponentCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneComponentFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMaterial {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMaterialFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMaterialInput {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMaterialInputFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMesh {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bounds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub PrimitiveTopology: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPrimitiveTopology: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPrimitiveTopology: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    pub FillMeshAttribute: unsafe extern "system" fn(this: *mut *mut Self, semantic: SceneAttributeSemantic, format: super::super::super::Graphics::DirectX::DirectXPixelFormat, memory: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))]
    FillMeshAttribute: usize,
}
#[repr(C)]
pub struct ISceneMeshMaterialAttributeMap {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneMeshRendererComponent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Material: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Mesh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UVMappings: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneMeshRendererComponentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneMeshStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneMetallicRoughnessMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub BaseColorInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBaseColorInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub BaseColorFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BaseColorFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetBaseColorFactor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetBaseColorFactor: usize,
    pub MetallicFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetMetallicFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub MetallicRoughnessInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMetallicRoughnessInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RoughnessFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRoughnessFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneMetallicRoughnessMaterialStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneModelTransform {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Quaternion) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub RotationAngle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub RotationAxis: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetRotationAxis: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetRotationAxis: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Scale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Scale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetScale: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Translation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Translation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTranslation: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTranslation: usize,
}
#[repr(C)]
pub struct ISceneNode {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub FindFirstComponentOfType: unsafe extern "system" fn(this: *mut *mut Self, value: SceneComponentType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneNodeCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneNodeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneObject {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IScenePbrMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub AlphaCutoff: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetAlphaCutoff: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneAlphaMode) -> ::windows_sys::core::HRESULT,
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, value: SceneAlphaMode) -> ::windows_sys::core::HRESULT,
    pub EmissiveInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetEmissiveInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub EmissiveFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EmissiveFactor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEmissiveFactor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEmissiveFactor: usize,
    pub IsDoubleSided: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDoubleSided: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub NormalInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNormalInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NormalScale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetNormalScale: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub OcclusionInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOcclusionInput: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OcclusionStrength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetOcclusionStrength: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IScenePbrMaterialFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneRendererComponent {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneRendererComponentFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ISceneSurfaceMaterialInput {
    pub base__: ::windows_sys::core::IInspectable,
    pub BitmapInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::CompositionBitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut *mut Self, value: super::CompositionBitmapInterpolationMode) -> ::windows_sys::core::HRESULT,
    pub Surface: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub WrappingUMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneWrappingMode) -> ::windows_sys::core::HRESULT,
    pub SetWrappingUMode: unsafe extern "system" fn(this: *mut *mut Self, value: SceneWrappingMode) -> ::windows_sys::core::HRESULT,
    pub WrappingVMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneWrappingMode) -> ::windows_sys::core::HRESULT,
    pub SetWrappingVMode: unsafe extern "system" fn(this: *mut *mut Self, value: SceneWrappingMode) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneSurfaceMaterialInputStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Root: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISceneVisualStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
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
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
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
pub type SceneBoundingBox = *mut ::core::ffi::c_void;
pub type SceneComponent = *mut ::core::ffi::c_void;
pub type SceneComponentCollection = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
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
pub type SceneMaterial = *mut ::core::ffi::c_void;
pub type SceneMaterialInput = *mut ::core::ffi::c_void;
pub type SceneMesh = *mut ::core::ffi::c_void;
pub type SceneMeshMaterialAttributeMap = *mut ::core::ffi::c_void;
pub type SceneMeshRendererComponent = *mut ::core::ffi::c_void;
pub type SceneMetallicRoughnessMaterial = *mut ::core::ffi::c_void;
pub type SceneModelTransform = *mut ::core::ffi::c_void;
pub type SceneNode = *mut ::core::ffi::c_void;
pub type SceneNodeCollection = *mut ::core::ffi::c_void;
pub type SceneObject = *mut ::core::ffi::c_void;
pub type ScenePbrMaterial = *mut ::core::ffi::c_void;
pub type SceneRendererComponent = *mut ::core::ffi::c_void;
pub type SceneSurfaceMaterialInput = *mut ::core::ffi::c_void;
pub type SceneVisual = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
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
