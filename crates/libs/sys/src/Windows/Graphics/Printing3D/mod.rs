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
impl ::windows_sys::core::Interface for IPrint3DManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1294977802, data2: 29542, data3: 18801, data4: [139, 213, 23, 196, 227, 232, 198, 192] };
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
impl ::windows_sys::core::Interface for IPrint3DManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 250727166, data2: 43437, data3: 19464, data4: [169, 23, 29, 31, 134, 62, 171, 203] };
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
impl ::windows_sys::core::Interface for IPrint3DTask {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2363740288, data2: 8472, data3: 19496, data4: [128, 222, 244, 38, 215, 1, 145, 174] };
}
#[repr(C)]
pub struct IPrint3DTaskCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Completion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DTaskCompletion) -> ::windows_sys::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Print3DTaskDetail) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DTaskCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3424195759, data2: 9748, data3: 20253, data4: [172, 204, 214, 252, 79, 218, 84, 85] };
}
#[repr(C)]
pub struct IPrint3DTaskRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTask: unsafe extern "system" fn(this: *mut *mut Self, title: ::windows_sys::core::HSTRING, printerid: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DTaskRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630572143, data2: 8773, data3: 19546, data4: [135, 49, 13, 96, 77, 198, 188, 60] };
}
#[repr(C)]
pub struct IPrint3DTaskRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DTaskRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353154943, data2: 6341, data3: 16599, data4: [159, 64, 250, 179, 9, 110, 5, 169] };
}
#[repr(C)]
pub struct IPrint3DTaskSourceChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DTaskSourceChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1540175023, data2: 9449, data3: 19472, data4: [141, 7, 20, 195, 70, 186, 63, 207] };
}
#[repr(C)]
pub struct IPrint3DTaskSourceRequestedArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetSource: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DTaskSourceRequestedArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3346832058, data2: 9391, data3: 16973, data4: [163, 191, 146, 37, 12, 53, 86, 2] };
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
impl ::windows_sys::core::Interface for IPrinting3D3MFPackage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4132296136, data2: 10935, data3: 17833, data4: [161, 183, 38, 126, 148, 141, 91, 24] };
}
#[repr(C)]
pub struct IPrinting3D3MFPackage2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Compression: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Printing3DPackageCompression) -> ::windows_sys::core::HRESULT,
    pub SetCompression: unsafe extern "system" fn(this: *mut *mut Self, value: Printing3DPackageCompression) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3D3MFPackage2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2522643140, data2: 37835, data3: 17456, data4: [146, 184, 120, 156, 212, 84, 248, 131] };
}
#[repr(C)]
pub struct IPrinting3D3MFPackageStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
}
impl ::windows_sys::core::Interface for IPrinting3D3MFPackageStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1884871087, data2: 31386, data3: 18311, data4: [184, 23, 246, 244, 89, 33, 72, 35] };
}
#[repr(C)]
pub struct IPrinting3DBaseMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DBaseMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3505448771, data2: 50444, data3: 19403, data4: [157, 4, 252, 22, 173, 206, 162, 201] };
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
impl ::windows_sys::core::Interface for IPrinting3DBaseMaterialGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2498785464, data2: 9493, data3: 19085, data4: [161, 240, 208, 252, 19, 208, 96, 33] };
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DBaseMaterialGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1544898268, data2: 34455, data3: 16787, data4: [151, 107, 132, 187, 65, 22, 229, 191] };
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Abs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Pla: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DBaseMaterialStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2170177468, data2: 14154, data3: 18285, data4: [190, 146, 62, 207, 209, 203, 151, 118] };
}
#[repr(C)]
pub struct IPrinting3DColorMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DColorMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3783891240, data2: 31975, data3: 17029, data4: [163, 93, 241, 69, 201, 81, 12, 123] };
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
impl ::windows_sys::core::Interface for IPrinting3DColorMaterial2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4205897810, data2: 2799, data3: 17641, data4: [157, 221, 54, 238, 234, 90, 205, 68] };
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
impl ::windows_sys::core::Interface for IPrinting3DColorMaterialGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1731536, data2: 43743, data3: 16934, data4: [175, 233, 243, 105, 160, 180, 80, 4] };
}
#[repr(C)]
pub struct IPrinting3DColorMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DColorMaterialGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1909689709, data2: 45546, data3: 19035, data4: [188, 84, 25, 198, 95, 61, 240, 68] };
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
impl ::windows_sys::core::Interface for IPrinting3DComponent {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2116581445, data2: 49023, data3: 19675, data4: [162, 127, 48, 160, 20, 55, 254, 222] };
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
impl ::windows_sys::core::Interface for IPrinting3DComponentWithMatrix {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 846852917, data2: 3824, data3: 17771, data4: [154, 33, 73, 190, 190, 139, 81, 194] };
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
}
impl ::windows_sys::core::Interface for IPrinting3DCompositeMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1176647901, data2: 22062, data3: 20332, data4: [136, 45, 244, 216, 65, 253, 99, 199] };
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
impl ::windows_sys::core::Interface for IPrinting3DCompositeMaterialGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2375314011, data2: 16625, data3: 18797, data4: [165, 251, 52, 10, 90, 103, 142, 48] };
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroup2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub BaseMaterialGroup: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetBaseMaterialGroup: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DCompositeMaterialGroup2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 115895650, data2: 32059, data3: 16865, data4: [148, 76, 186, 253, 228, 85, 84, 131] };
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DCompositeMaterialGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3499019539, data2: 37631, data3: 17322, data4: [166, 39, 141, 67, 194, 44, 129, 126] };
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
impl ::windows_sys::core::Interface for IPrinting3DFaceReductionOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3154039703, data2: 11636, data3: 18167, data4: [190, 133, 153, 166, 123, 187, 102, 41] };
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
impl ::windows_sys::core::Interface for IPrinting3DMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 932033110, data2: 60770, data3: 18770, data4: [184, 91, 3, 86, 125, 124, 70, 94] };
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
impl ::windows_sys::core::Interface for IPrinting3DMesh {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 422482140, data2: 552, data3: 11777, data4: [188, 32, 197, 41, 12, 191, 50, 196] };
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
impl ::windows_sys::core::Interface for IPrinting3DMeshVerificationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 425095610, data2: 59706, data3: 20106, data4: [164, 111, 222, 168, 232, 82, 25, 126] };
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
impl ::windows_sys::core::Interface for IPrinting3DModel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 755052272, data2: 21243, data3: 37274, data4: [119, 176, 75, 26, 59, 128, 50, 79] };
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
impl ::windows_sys::core::Interface for IPrinting3DModel2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3374344647, data2: 51265, data3: 18419, data4: [168, 78, 161, 73, 253, 8, 182, 87] };
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
impl ::windows_sys::core::Interface for IPrinting3DModelTexture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1571802881, data2: 46493, data3: 18492, data4: [151, 187, 164, 213, 70, 209, 199, 92] };
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterial {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
impl ::windows_sys::core::Interface for IPrinting3DMultiplePropertyMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 631645515, data2: 50921, data3: 18509, data4: [162, 20, 162, 94, 87, 118, 186, 98] };
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
impl ::windows_sys::core::Interface for IPrinting3DMultiplePropertyMaterialGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4036298009, data2: 44729, data3: 17685, data4: [163, 155, 160, 136, 251, 187, 39, 124] };
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DMultiplePropertyMaterialGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 842930542, data2: 54470, data3: 17694, data4: [168, 20, 77, 120, 162, 16, 254, 83] };
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
impl ::windows_sys::core::Interface for IPrinting3DTexture2CoordMaterial {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2374257659, data2: 2025, data3: 18822, data4: [152, 51, 141, 211, 212, 140, 104, 89] };
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
impl ::windows_sys::core::Interface for IPrinting3DTexture2CoordMaterialGroup {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1652391079, data2: 28048, data3: 20409, data4: [159, 196, 159, 239, 243, 223, 168, 146] };
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroup2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Texture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DTexture2CoordMaterialGroup2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1778113466, data2: 45358, data3: 17051, data4: [131, 134, 223, 82, 132, 246, 232, 15] };
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrinting3DTexture2CoordMaterialGroupFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3417328048, data2: 18058, data3: 19567, data4: [178, 162, 142, 184, 186, 141, 234, 72] };
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
impl ::windows_sys::core::Interface for IPrinting3DTextureResource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2802709293, data2: 27313, data3: 17582, data4: [188, 69, 162, 115, 130, 192, 211, 140] };
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
