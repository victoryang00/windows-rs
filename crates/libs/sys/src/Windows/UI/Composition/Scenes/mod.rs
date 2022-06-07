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
impl ::windows_sys::core::Interface for ISceneBoundingBox {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1569717360, data2: 50712, data3: 16515, data4: [130, 81, 153, 98, 89, 49, 20, 170] };
}
#[repr(C)]
pub struct ISceneComponent {
    pub base__: ::windows_sys::core::IInspectable,
    pub ComponentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SceneComponentType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneComponent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2921397398, data2: 8812, data3: 17597, data4: [149, 203, 221, 94, 217, 235, 233, 165] };
}
#[repr(C)]
pub struct ISceneComponentCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneComponentCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3296950556, data2: 24390, data3: 17892, data4: [182, 102, 163, 210, 37, 159, 155, 46] };
}
#[repr(C)]
pub struct ISceneComponentFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneComponentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1606178164, data2: 56792, data3: 22665, data4: [171, 91, 216, 250, 113, 110, 124, 158] };
}
#[repr(C)]
pub struct ISceneMaterial {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2359774076, data2: 12511, data3: 19975, data4: [148, 144, 55, 135, 90, 241, 161, 35] };
}
#[repr(C)]
pub struct ISceneMaterialFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneMaterialFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1733520409, data2: 42759, data3: 21076, data4: [164, 149, 127, 220, 121, 152, 147, 185] };
}
#[repr(C)]
pub struct ISceneMaterialInput {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneMaterialInput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1110054466, data2: 7921, data3: 18524, data4: [151, 233, 174, 111, 149, 173, 129, 47] };
}
#[repr(C)]
pub struct ISceneMaterialInputFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneMaterialInputFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2828004212, data2: 32010, data3: 24140, data4: [167, 72, 16, 21, 175, 156, 167, 79] };
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
impl ::windows_sys::core::Interface for ISceneMesh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003075376, data2: 4437, data3: 19468, data4: [146, 189, 64, 2, 12, 247, 131, 71] };
}
#[repr(C)]
pub struct ISceneMeshMaterialAttributeMap {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneMeshMaterialAttributeMap {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3464769905, data2: 15683, data3: 18517, data4: [170, 105, 49, 255, 152, 141, 4, 157] };
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
impl ::windows_sys::core::Interface for ISceneMeshRendererComponent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2569664483, data2: 25444, data3: 18302, data4: [152, 254, 116, 237, 159, 212, 194, 222] };
}
#[repr(C)]
pub struct ISceneMeshRendererComponentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneMeshRendererComponentStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1230304122, data2: 17497, data3: 17697, data4: [189, 110, 43, 56, 184, 215, 17, 234] };
}
#[repr(C)]
pub struct ISceneMeshStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneMeshStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2215784812, data2: 31575, data3: 18239, data4: [150, 107, 129, 220, 39, 123, 23, 81] };
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
impl ::windows_sys::core::Interface for ISceneMetallicRoughnessMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3252229190, data2: 31132, data3: 17054, data4: [164, 228, 93, 166, 69, 241, 142, 97] };
}
#[repr(C)]
pub struct ISceneMetallicRoughnessMaterialStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneMetallicRoughnessMaterialStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1004390992, data2: 28061, data3: 17713, data4: [141, 196, 178, 126, 62, 73, 183, 171] };
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
impl ::windows_sys::core::Interface for ISceneModelTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3226826434, data2: 12977, data3: 17001, data4: [152, 13, 185, 133, 55, 16, 10, 228] };
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
impl ::windows_sys::core::Interface for ISceneNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2901590599, data2: 62215, data3: 17793, data4: [156, 65, 175, 46, 41, 195, 176, 22] };
}
#[repr(C)]
pub struct ISceneNodeCollection {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneNodeCollection {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 699244801, data2: 11737, data3: 17202, data4: [190, 99, 96, 210, 207, 66, 105, 242] };
}
#[repr(C)]
pub struct ISceneNodeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneNodeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1469714346, data2: 48797, data3: 16912, data4: [144, 140, 147, 209, 95, 238, 208, 183] };
}
#[repr(C)]
pub struct ISceneObject {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 513025179, data2: 3867, data3: 18923, data4: [168, 25, 135, 125, 132, 80, 0, 91] };
}
#[repr(C)]
pub struct ISceneObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneObjectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 352221594, data2: 13284, data3: 21231, data4: [149, 108, 68, 34, 157, 33, 242, 193] };
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
impl ::windows_sys::core::Interface for IScenePbrMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2864114622, data2: 54912, data3: 18143, data4: [130, 148, 182, 128, 10, 159, 149, 231] };
}
#[repr(C)]
pub struct IScenePbrMaterialFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IScenePbrMaterialFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 775896574, data2: 2949, data3: 22311, data4: [181, 190, 183, 211, 203, 172, 55, 250] };
}
#[repr(C)]
pub struct ISceneRendererComponent {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneRendererComponent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4054628439, data2: 53071, data3: 16421, data4: [155, 37, 162, 209, 148, 76, 245, 7] };
}
#[repr(C)]
pub struct ISceneRendererComponentFactory {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ISceneRendererComponentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 498527596, data2: 43564, data3: 22887, data4: [144, 53, 86, 53, 45, 198, 150, 88] };
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
impl ::windows_sys::core::Interface for ISceneSurfaceMaterialInput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2570574428, data2: 43466, data3: 19708, data4: [179, 170, 8, 131, 86, 81, 135, 66] };
}
#[repr(C)]
pub struct ISceneSurfaceMaterialInputStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneSurfaceMaterialInputStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1512281299, data2: 25641, data3: 17801, data4: [187, 207, 184, 79, 79, 60, 251, 254] };
}
#[repr(C)]
pub struct ISceneVisual {
    pub base__: ::windows_sys::core::IInspectable,
    pub Root: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneVisual {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2389126174, data2: 55092, data3: 18353, data4: [190, 20, 61, 105, 79, 250, 67, 1] };
}
#[repr(C)]
pub struct ISceneVisualStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISceneVisualStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3090448026, data2: 20650, data3: 17703, data4: [141, 52, 222, 76, 184, 234, 136, 180] };
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
