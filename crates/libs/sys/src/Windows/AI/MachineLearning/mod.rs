#[cfg(feature = "AI_MachineLearning_Preview")]
pub mod Preview;
#[repr(C)]
pub struct IImageFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapAlphaMode: usize,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageFeatureDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 911574437, data2: 5914, data3: 18986, data4: [152, 95, 38, 81, 89, 211, 137, 90] };
}
#[repr(C)]
pub struct IImageFeatureDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelPixelRange) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IImageFeatureDescriptor2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 724028583, data2: 54579, data3: 22626, data4: [187, 152, 22, 17, 177, 85, 176, 225] };
}
#[repr(C)]
pub struct IImageFeatureValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
impl ::windows_sys::core::Interface for IImageFeatureValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4030812121, data2: 51626, data3: 17413, data4: [183, 251, 148, 248, 124, 138, 48, 55] };
}
#[repr(C)]
pub struct IImageFeatureValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
impl ::windows_sys::core::Interface for IImageFeatureValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 465770493, data2: 9163, data3: 17936, data4: [176, 133, 200, 225, 200, 126, 186, 160] };
}
#[repr(C)]
pub struct ILearningModel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InputFeatures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InputFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutputFeatures: usize,
}
impl ::windows_sys::core::Interface for ILearningModel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1536051488, data2: 18591, data3: 20102, data4: [145, 40, 38, 90, 50, 123, 120, 250] };
}
#[repr(C)]
pub struct ILearningModelBinding {
    pub base__: ::windows_sys::core::IInspectable,
    pub Bind: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void, props: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelBinding {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3929091872, data2: 5775, data3: 20364, data4: [148, 254, 46, 122, 195, 27, 74, 168] };
}
#[repr(C)]
pub struct ILearningModelBindingFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromSession: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelBindingFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3378477690, data2: 59272, data3: 18270, data4: [137, 23, 35, 170, 56, 31, 175, 11] };
}
#[repr(C)]
pub struct ILearningModelDevice {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
impl ::windows_sys::core::Interface for ILearningModelDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4123183358, data2: 16214, data3: 19084, data4: [172, 95, 253, 185, 45, 139, 130, 82] };
}
#[repr(C)]
pub struct ILearningModelDeviceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, devicekind: LearningModelDeviceKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelDeviceFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2634012493, data2: 45541, data3: 20256, data4: [128, 173, 10, 86, 105, 13, 176, 107] };
}
#[repr(C)]
pub struct ILearningModelDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
impl ::windows_sys::core::Interface for ILearningModelDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1240670471, data2: 43199, data3: 17083, data4: [146, 199, 16, 177, 45, 197, 210, 31] };
}
#[repr(C)]
pub struct ILearningModelEvaluationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Outputs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Outputs: usize,
}
impl ::windows_sys::core::Interface for ILearningModelEvaluationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3002712013, data2: 38414, data3: 18880, data4: [133, 147, 235, 25, 10, 227, 238, 226] };
}
#[repr(C)]
pub struct ILearningModelFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelFeatureKind) -> ::windows_sys::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelFeatureDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3154694012, data2: 28368, data3: 16388, data4: [151, 186, 185, 162, 238, 205, 43, 79] };
}
#[repr(C)]
pub struct ILearningModelFeatureValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelFeatureKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelFeatureValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4111467995, data2: 16517, data3: 19966, data4: [159, 237, 149, 235, 12, 12, 247, 92] };
}
#[repr(C)]
pub struct ILearningModelOperatorProvider {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for ILearningModelOperatorProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 706883165, data2: 44977, data3: 18413, data4: [191, 173, 181, 179, 164, 89, 236, 4] };
}
#[repr(C)]
pub struct ILearningModelSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Model: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    #[cfg(feature = "Foundation")]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut *mut Self, bindings: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EvaluateAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut *mut Self, features: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeaturesAsync: usize,
    pub Evaluate: unsafe extern "system" fn(this: *mut *mut Self, bindings: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeatures: unsafe extern "system" fn(this: *mut *mut Self, features: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeatures: usize,
}
impl ::windows_sys::core::Interface for ILearningModelSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2388195574, data2: 46983, data3: 19473, data4: [144, 240, 113, 41, 174, 202, 116, 169] };
}
#[repr(C)]
pub struct ILearningModelSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromModel: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 258705437, data2: 7323, data3: 18358, data4: [191, 224, 241, 207, 98, 166, 117, 121] };
}
#[repr(C)]
pub struct ILearningModelSessionFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, learningmodelsessionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1314687167, data2: 2591, data3: 24556, data4: [173, 224, 47, 217, 30, 78, 242, 155] };
}
#[repr(C)]
pub struct ILearningModelSessionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub BatchSizeOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3103145889, data2: 4941, data3: 20787, data4: [140, 255, 58, 92, 60, 38, 59, 235] };
}
#[repr(C)]
pub struct ILearningModelSessionOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionOptions2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1875713476, data2: 5983, data3: 23506, data4: [141, 229, 47, 32, 6, 162, 90, 223] };
}
#[repr(C)]
pub struct ILearningModelSessionOptions3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverrideNamedDimension: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, dimension: u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILearningModelSessionOptions3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1491164398, data2: 55490, data3: 22268, data4: [146, 232, 118, 215, 81, 8, 16, 134] };
}
#[repr(C)]
pub struct ILearningModelStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, modelfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(this: *mut *mut Self, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut *mut Self, modelfile: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(this: *mut *mut Self, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(this: *mut *mut Self, filepath: ::windows_sys::core::HSTRING, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(this: *mut *mut Self, modelstream: *mut ::core::ffi::c_void, operatorprovider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
impl ::windows_sys::core::Interface for ILearningModelStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820582888, data2: 26962, data3: 20039, data4: [142, 244, 31, 127, 7, 137, 124, 109] };
}
#[repr(C)]
pub struct IMapFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TensorKind) -> ::windows_sys::core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMapFeatureDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1392780477, data2: 41559, data3: 17261, data4: [158, 96, 194, 152, 31, 124, 197, 196] };
}
#[repr(C)]
pub struct ISequenceFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ElementDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISequenceFeatureDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2230752346, data2: 22059, data3: 19810, data4: [168, 81, 115, 154, 206, 217, 102, 104] };
}
#[repr(C)]
pub struct ITensor {
    pub base__: ::windows_sys::core::IInspectable,
    pub TensorKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TensorKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
impl ::windows_sys::core::Interface for ITensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 88642963, data2: 41733, data3: 18981, data4: [173, 9, 68, 1, 25, 180, 183, 246] };
}
#[repr(C)]
pub struct ITensorBoolean {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorBoolean {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1358107117, data2: 10729, data3: 19036, data4: [164, 77, 143, 197, 18, 88, 78, 237] };
}
#[repr(C)]
pub struct ITensorBooleanStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorBooleanStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 664176172, data2: 9047, data3: 18855, data4: [180, 118, 208, 170, 61, 254, 104, 102] };
}
#[repr(C)]
pub struct ITensorBooleanStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorBooleanStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2745476353, data2: 27181, data3: 21207, data4: [176, 75, 196, 53, 186, 238, 1, 21] };
}
#[repr(C)]
pub struct ITensorDouble {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorDouble {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2447643218, data2: 31375, data3: 20238, data4: [162, 143, 150, 55, 255, 200, 163, 208] };
}
#[repr(C)]
pub struct ITensorDoubleStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorDoubleStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2825294789, data2: 38200, data3: 17639, data4: [163, 202, 93, 243, 116, 165, 167, 12] };
}
#[repr(C)]
pub struct ITensorDoubleStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorDoubleStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2477093086, data2: 24218, data3: 20628, data4: [133, 200, 89, 44, 101, 94, 104, 172] };
}
#[repr(C)]
pub struct ITensorFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub TensorKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TensorKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
impl ::windows_sys::core::Interface for ITensorFeatureDescriptor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1950702720, data2: 37994, data3: 17168, data4: [161, 156, 238, 10, 240, 40, 252, 228] };
}
#[repr(C)]
pub struct ITensorFloat {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorFloat {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4062719362, data2: 43522, data3: 17096, data4: [160, 200, 223, 30, 252, 150, 118, 225] };
}
#[repr(C)]
pub struct ITensorFloat16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorFloat16Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 179934460, data2: 23433, data3: 19516, data4: [181, 228, 82, 130, 165, 49, 108, 10] };
}
#[repr(C)]
pub struct ITensorFloat16BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorFloat16BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2771236597, data2: 12682, data3: 17620, data4: [130, 11, 12, 220, 112, 84, 168, 74] };
}
#[repr(C)]
pub struct ITensorFloat16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorFloat16BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750357798, data2: 11719, data3: 20927, data4: [180, 112, 11, 52, 76, 194, 161, 188] };
}
#[repr(C)]
pub struct ITensorFloatStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorFloatStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687659867, data2: 15267, data3: 17711, data4: [177, 13, 60, 19, 94, 87, 63, 169] };
}
#[repr(C)]
pub struct ITensorFloatStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorFloatStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 610339777, data2: 24132, data3: 22291, data4: [178, 129, 143, 74, 212, 213, 85, 232] };
}
#[repr(C)]
pub struct ITensorInt16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorInt16Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2560830777, data2: 59094, data3: 17583, data4: [138, 250, 186, 235, 196, 77, 192, 32] };
}
#[repr(C)]
pub struct ITensorInt16BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorInt16BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2556715667, data2: 9838, data3: 19226, data4: [130, 31, 230, 13, 112, 137, 139, 145] };
}
#[repr(C)]
pub struct ITensorInt16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorInt16BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 215420148, data2: 26988, data3: 24159, data4: [149, 216, 94, 191, 150, 112, 20, 139] };
}
#[repr(C)]
pub struct ITensorInt32Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorInt32Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 738994387, data2: 8316, data3: 17542, data4: [167, 210, 136, 69, 34, 197, 229, 137] };
}
#[repr(C)]
pub struct ITensorInt32BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorInt32BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1698268747, data2: 21242, data3: 20021, data4: [144, 124, 131, 76, 172, 65, 123, 80] };
}
#[repr(C)]
pub struct ITensorInt32BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorInt32BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2085291930, data2: 59734, data3: 23776, data4: [163, 189, 21, 125, 157, 121, 181, 236] };
}
#[repr(C)]
pub struct ITensorInt64Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorInt64Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1234593210, data2: 8098, data3: 17837, data4: [175, 37, 160, 189, 155, 218, 76, 135] };
}
#[repr(C)]
pub struct ITensorInt64BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorInt64BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2521345437, data2: 4504, data3: 19828, data4: [149, 23, 120, 58, 182, 43, 156, 194] };
}
#[repr(C)]
pub struct ITensorInt64BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorInt64BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1832754635, data2: 65344, data3: 24258, data4: [137, 254, 8, 78, 43, 107, 198, 219] };
}
#[repr(C)]
pub struct ITensorInt8Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorInt8Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3453851589, data2: 65496, data3: 20463, data4: [174, 251, 48, 225, 164, 133, 178, 238] };
}
#[repr(C)]
pub struct ITensorInt8BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorInt8BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2980127364, data2: 2396, data3: 19574, data4: [166, 97, 172, 76, 238, 31, 62, 139] };
}
#[repr(C)]
pub struct ITensorInt8BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorInt8BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3235223095, data2: 50280, data3: 22267, data4: [149, 53, 192, 82, 189, 185, 61, 192] };
}
#[repr(C)]
pub struct ITensorString {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorString {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1478702536, data2: 48561, data3: 17936, data4: [188, 117, 53, 233, 203, 240, 9, 183] };
}
#[repr(C)]
pub struct ITensorStringStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorStringStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2204250916, data2: 53030, data3: 20247, data4: [162, 212, 32, 239, 141, 9, 125, 83] };
}
#[repr(C)]
pub struct ITensorStringStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITensorStringStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654297808, data2: 51426, data3: 21076, data4: [145, 55, 1, 147, 163, 102, 143, 216] };
}
#[repr(C)]
pub struct ITensorUInt16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt16Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1746145099, data2: 9152, data3: 17139, data4: [129, 246, 168, 145, 192, 17, 188, 63] };
}
#[repr(C)]
pub struct ITensorUInt16BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt16BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1576486365, data2: 650, data3: 18458, data4: [162, 124, 199, 230, 67, 94, 82, 221] };
}
#[repr(C)]
pub struct ITensorUInt16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt16BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2331249764, data2: 54943, data3: 21269, data4: [147, 72, 73, 8, 119, 187, 214, 66] };
}
#[repr(C)]
pub struct ITensorUInt32Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt32Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3637101311, data2: 29969, data3: 17827, data4: [191, 172, 195, 143, 55, 13, 34, 55] };
}
#[repr(C)]
pub struct ITensorUInt32BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt32BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1098659895, data2: 59251, data3: 17272, data4: [142, 127, 12, 195, 61, 190, 166, 151] };
}
#[repr(C)]
pub struct ITensorUInt32BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt32BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011466524, data2: 12622, data3: 22173, data4: [180, 150, 92, 132, 71, 210, 12, 210] };
}
#[repr(C)]
pub struct ITensorUInt64Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt64Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 779157421, data2: 1215, data3: 18469, data4: [131, 154, 130, 186, 239, 140, 120, 134] };
}
#[repr(C)]
pub struct ITensorUInt64BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt64BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2055086315, data2: 9263, data3: 18379, data4: [169, 198, 246, 2, 236, 251, 254, 228] };
}
#[repr(C)]
pub struct ITensorUInt64BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt64BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 140142717, data2: 26593, data3: 23326, data4: [178, 50, 79, 171, 233, 202, 32, 179] };
}
#[repr(C)]
pub struct ITensorUInt8Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt8Bit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1491185191, data2: 25131, data3: 18659, data4: [190, 34, 216, 103, 174, 209, 218, 172] };
}
#[repr(C)]
pub struct ITensorUInt8BitStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(this: *mut *mut Self, shape: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt8BitStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 100038019, data2: 48164, data3: 16928, data4: [138, 65, 45, 205, 140, 94, 211, 60] };
}
#[repr(C)]
pub struct ITensorUInt8BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
impl ::windows_sys::core::Interface for ITensorUInt8BitStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 731923158, data2: 14142, data3: 23098, data4: [162, 252, 166, 196, 27, 213, 39, 137] };
}
pub type ImageFeatureDescriptor = *mut ::core::ffi::c_void;
pub type ImageFeatureValue = *mut ::core::ffi::c_void;
pub type LearningModel = *mut ::core::ffi::c_void;
pub type LearningModelBinding = *mut ::core::ffi::c_void;
pub type LearningModelDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
impl ::core::marker::Copy for LearningModelDeviceKind {}
impl ::core::clone::Clone for LearningModelDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LearningModelEvaluationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for LearningModelFeatureKind {}
impl ::core::clone::Clone for LearningModelFeatureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
impl ::core::marker::Copy for LearningModelPixelRange {}
impl ::core::clone::Clone for LearningModelPixelRange {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LearningModelSession = *mut ::core::ffi::c_void;
pub type LearningModelSessionOptions = *mut ::core::ffi::c_void;
pub type MapFeatureDescriptor = *mut ::core::ffi::c_void;
pub type SequenceFeatureDescriptor = *mut ::core::ffi::c_void;
pub type TensorBoolean = *mut ::core::ffi::c_void;
pub type TensorDouble = *mut ::core::ffi::c_void;
pub type TensorFeatureDescriptor = *mut ::core::ffi::c_void;
pub type TensorFloat = *mut ::core::ffi::c_void;
pub type TensorFloat16Bit = *mut ::core::ffi::c_void;
pub type TensorInt16Bit = *mut ::core::ffi::c_void;
pub type TensorInt32Bit = *mut ::core::ffi::c_void;
pub type TensorInt64Bit = *mut ::core::ffi::c_void;
pub type TensorInt8Bit = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"AI_MachineLearning\"`*"]
#[repr(transparent)]
pub struct TensorKind(pub i32);
impl TensorKind {
    pub const Undefined: Self = Self(0i32);
    pub const Float: Self = Self(1i32);
    pub const UInt8: Self = Self(2i32);
    pub const Int8: Self = Self(3i32);
    pub const UInt16: Self = Self(4i32);
    pub const Int16: Self = Self(5i32);
    pub const Int32: Self = Self(6i32);
    pub const Int64: Self = Self(7i32);
    pub const String: Self = Self(8i32);
    pub const Boolean: Self = Self(9i32);
    pub const Float16: Self = Self(10i32);
    pub const Double: Self = Self(11i32);
    pub const UInt32: Self = Self(12i32);
    pub const UInt64: Self = Self(13i32);
    pub const Complex64: Self = Self(14i32);
    pub const Complex128: Self = Self(15i32);
}
impl ::core::marker::Copy for TensorKind {}
impl ::core::clone::Clone for TensorKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TensorString = *mut ::core::ffi::c_void;
pub type TensorUInt16Bit = *mut ::core::ffi::c_void;
pub type TensorUInt32Bit = *mut ::core::ffi::c_void;
pub type TensorUInt64Bit = *mut ::core::ffi::c_void;
pub type TensorUInt8Bit = *mut ::core::ffi::c_void;
