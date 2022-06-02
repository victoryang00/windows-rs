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
#[repr(C)]
pub struct IImageFeatureDescriptor2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelRange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelPixelRange) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IImageFeatureValue {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
#[repr(C)]
pub struct IImageFeatureValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(this: *mut *mut Self, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
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
#[repr(C)]
pub struct ILearningModelBindingFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromSession: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ILearningModelDeviceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, devicekind: LearningModelDeviceKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
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
#[repr(C)]
pub struct ILearningModelFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelFeatureKind) -> ::windows_sys::core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelFeatureValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelFeatureKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelOperatorProvider {
    pub base__: ::windows_sys::core::IInspectable,
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
#[repr(C)]
pub struct ILearningModelSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromModel: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelSessionFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, devicetorunon: *mut ::core::ffi::c_void, learningmodelsessionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelSessionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub BatchSizeOverride: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelSessionOptions2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILearningModelSessionOptions3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub OverrideNamedDimension: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, dimension: u32) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IMapFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub KeyKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TensorKind) -> ::windows_sys::core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISequenceFeatureDescriptor {
    pub base__: ::windows_sys::core::IInspectable,
    pub ElementDescriptor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ITensorBoolean {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorBooleanStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorDouble {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorDoubleStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
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
#[repr(C)]
pub struct ITensorFloat {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
#[repr(C)]
pub struct ITensorFloat16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorFloat16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
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
#[repr(C)]
pub struct ITensorFloatStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorInt16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorInt16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorInt32Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorInt32BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorInt64Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorInt64BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorInt8Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorInt8BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorString {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorStringStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITensorUInt16Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorUInt16BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorUInt32Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorUInt32BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorUInt64Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorUInt64BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(C)]
pub struct ITensorUInt8Bit {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
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
#[repr(C)]
pub struct ITensorUInt8BitStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, data_array_size: u32, data: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, shape_array_size: u32, shape: *const i64, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
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
