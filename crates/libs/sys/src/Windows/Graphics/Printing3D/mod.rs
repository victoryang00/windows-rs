#[repr(C)]
pub struct IPrint3DManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TaskRequested: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTaskRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTaskRequested: usize,
}
#[repr(C)]
pub struct IPrint3DManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowPrintUIAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowPrintUIAsync: usize,
}
#[repr(C)]
pub struct IPrint3DTask {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Submitting: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitting: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitting: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChanged: unsafe extern "system" fn(this: *mut *mut Self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChanged: usize,
}
#[repr(C)]
pub struct IPrint3DTaskCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Completion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DTaskCompletion) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DTaskDetail) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTask: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, printerid: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DTaskRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DTaskSourceChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrint3DTaskSourceRequestedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3D3MFPackage {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PrintTicket: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ModelPart: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ModelPart: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelPart: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelPart: usize,
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadModelFromPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadModelFromPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveModelToPackageAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveModelToPackageAsync: usize,
}
#[repr(C)]
pub struct IPrinting3D3MFPackage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compression: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DPackageCompression) -> ::windows_sys::core::HRESULT,
    pub SetCompression: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DPackageCompression) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3D3MFPackageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
}
#[repr(C)]
pub struct IPrinting3DBaseMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Bases: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bases: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Abs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pla: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DColorMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DColorMaterial2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::UI::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
}
#[repr(C)]
pub struct IPrinting3DColorMaterialGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Colors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Colors: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DColorMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DComponent {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mesh: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DObjectType) -> ::windows_sys::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DObjectType) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PartNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetPartNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DComponentWithMatrix {
    pub base__: ::windows_sys::core::IInspectable,
    pub Component: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetComponent: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Matrix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Matrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Composites: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Composites: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroup2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BaseMaterialGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBaseMaterialGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DFaceReductionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub MaxReductionArea: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxReductionArea: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TargetTriangleCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetTargetTriangleCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub MaxEdgeLength: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetMaxEdgeLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub BaseGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BaseGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ColorGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ColorGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2CoordGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2CoordGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositeGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositeGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MultiplePropertyGroups: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultiplePropertyGroups: usize,
}
#[repr(C)]
pub struct IPrinting3DMesh {
    pub base__: ::windows_sys::core::IInspectable,
    pub VertexCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetVertexCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub IndexCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetIndexCount: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub VertexPositionsDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub SetVertexPositionsDescription: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub VertexNormalsDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub SetVertexNormalsDescription: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub TriangleIndicesDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub SetTriangleIndicesDescription: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub TriangleMaterialIndicesDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    pub SetTriangleMaterialIndicesDescription: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DBufferDescription) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexPositions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexPositions: usize,
    pub CreateVertexPositions: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexNormals: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexNormals: usize,
    pub CreateVertexNormals: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleIndices: usize,
    pub CreateTriangleIndices: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleMaterialIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleMaterialIndices: usize,
    pub CreateTriangleMaterialIndices: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferDescriptionSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferDescriptionSet: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferSet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferSet: usize,
    #[cfg(feature = "Foundation")]
    pub VerifyAsync: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DMeshVerificationMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerifyAsync: usize,
}
#[repr(C)]
pub struct IPrinting3DMeshVerificationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsValid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub NonmanifoldTriangles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NonmanifoldTriangles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReversedNormalTriangles: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReversedNormalTriangles: usize,
}
#[repr(C)]
pub struct IPrinting3DModel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DModelUnit) -> ::windows_sys::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DModelUnit) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Meshes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Meshes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Material: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Build: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBuild: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequiredExtensions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequiredExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation")]
    pub RepairAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepairAsync: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DModel2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TryPartialRepairAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPartialRepairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPartialRepairWithTimeAsync: unsafe extern "system" fn(this: *mut *mut Self, maxwaittime: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPartialRepairWithTimeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesWithOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, printing3dfacereductionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesWithOptionsAndTimeAsync: unsafe extern "system" fn(this: *mut *mut Self, printing3dfacereductionoptions: *mut ::core::ffi::c_void, maxwait: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesWithOptionsAndTimeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RepairWithProgressAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepairWithProgressAsync: usize,
}
#[repr(C)]
pub struct IPrinting3DModelTexture {
    pub base__: ::windows_sys::core::IInspectable,
    pub TextureResource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTextureResource: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TileStyleU: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows_sys::core::HRESULT,
    pub SetTileStyleU: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DTextureEdgeBehavior) -> ::windows_sys::core::HRESULT,
    pub TileStyleV: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows_sys::core::HRESULT,
    pub SetTileStyleV: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DTextureEdgeBehavior) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterialGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub MultipleProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultipleProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialGroupIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialGroupIndices: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub Texture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub U: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetU: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub V: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetV: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroup {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2Coords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2Coords: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroup2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Texture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrinting3DTextureResource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage_Streams")]
    pub TextureData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TextureData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextureData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextureData: usize,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
pub type Print3DManager = *mut ::core::ffi::c_void;
pub type Print3DTask = *mut ::core::ffi::c_void;
pub type Print3DTaskCompletedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskCompletion(pub i32);
impl Print3DTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl ::core::marker::Copy for Print3DTaskCompletion {}
impl ::core::clone::Clone for Print3DTaskCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl ::core::marker::Copy for Print3DTaskDetail {}
impl ::core::clone::Clone for Print3DTaskDetail {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Print3DTaskRequest = *mut ::core::ffi::c_void;
pub type Print3DTaskRequestedEventArgs = *mut ::core::ffi::c_void;
pub type Print3DTaskSourceChangedEventArgs = *mut ::core::ffi::c_void;
pub type Print3DTaskSourceRequestedArgs = *mut ::core::ffi::c_void;
pub type Print3DTaskSourceRequestedHandler = *mut ::core::ffi::c_void;
pub type Printing3D3MFPackage = *mut ::core::ffi::c_void;
pub type Printing3DBaseMaterial = *mut ::core::ffi::c_void;
pub type Printing3DBaseMaterialGroup = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
pub struct Printing3DBufferDescription {
    pub Format: Printing3DBufferFormat,
    pub Stride: u32,
}
impl ::core::marker::Copy for Printing3DBufferDescription {}
impl ::core::clone::Clone for Printing3DBufferDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DBufferFormat(pub i32);
impl Printing3DBufferFormat {
    pub const Unknown: Self = Self(0i32);
    pub const R32G32B32A32Float: Self = Self(2i32);
    pub const R32G32B32A32UInt: Self = Self(3i32);
    pub const R32G32B32Float: Self = Self(6i32);
    pub const R32G32B32UInt: Self = Self(7i32);
    pub const Printing3DDouble: Self = Self(500i32);
    pub const Printing3DUInt: Self = Self(501i32);
}
impl ::core::marker::Copy for Printing3DBufferFormat {}
impl ::core::clone::Clone for Printing3DBufferFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Printing3DColorMaterial = *mut ::core::ffi::c_void;
pub type Printing3DColorMaterialGroup = *mut ::core::ffi::c_void;
pub type Printing3DComponent = *mut ::core::ffi::c_void;
pub type Printing3DComponentWithMatrix = *mut ::core::ffi::c_void;
pub type Printing3DCompositeMaterial = *mut ::core::ffi::c_void;
pub type Printing3DCompositeMaterialGroup = *mut ::core::ffi::c_void;
pub type Printing3DFaceReductionOptions = *mut ::core::ffi::c_void;
pub type Printing3DMaterial = *mut ::core::ffi::c_void;
pub type Printing3DMesh = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMeshVerificationMode(pub i32);
impl Printing3DMeshVerificationMode {
    pub const FindFirstError: Self = Self(0i32);
    pub const FindAllErrors: Self = Self(1i32);
}
impl ::core::marker::Copy for Printing3DMeshVerificationMode {}
impl ::core::clone::Clone for Printing3DMeshVerificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Printing3DMeshVerificationResult = *mut ::core::ffi::c_void;
pub type Printing3DModel = *mut ::core::ffi::c_void;
pub type Printing3DModelTexture = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DModelUnit(pub i32);
impl Printing3DModelUnit {
    pub const Meter: Self = Self(0i32);
    pub const Micron: Self = Self(1i32);
    pub const Millimeter: Self = Self(2i32);
    pub const Centimeter: Self = Self(3i32);
    pub const Inch: Self = Self(4i32);
    pub const Foot: Self = Self(5i32);
}
impl ::core::marker::Copy for Printing3DModelUnit {}
impl ::core::clone::Clone for Printing3DModelUnit {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Printing3DMultiplePropertyMaterial = *mut ::core::ffi::c_void;
pub type Printing3DMultiplePropertyMaterialGroup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Self = Self(0i32);
    pub const Support: Self = Self(1i32);
    pub const Others: Self = Self(2i32);
}
impl ::core::marker::Copy for Printing3DObjectType {}
impl ::core::clone::Clone for Printing3DObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for Printing3DPackageCompression {}
impl ::core::clone::Clone for Printing3DPackageCompression {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Printing3DTexture2CoordMaterial = *mut ::core::ffi::c_void;
pub type Printing3DTexture2CoordMaterialGroup = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
    pub const Clamp: Self = Self(3i32);
}
impl ::core::marker::Copy for Printing3DTextureEdgeBehavior {}
impl ::core::clone::Clone for Printing3DTextureEdgeBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Printing3DTextureResource = *mut ::core::ffi::c_void;
