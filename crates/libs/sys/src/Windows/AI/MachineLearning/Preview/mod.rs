#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FeatureElementKindPreview(pub i32);
#[cfg(feature = "deprecated")]
impl FeatureElementKindPreview {
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
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for FeatureElementKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FeatureElementKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IImageVariableDescriptorPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Graphics_Imaging", feature = "deprecated"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Imaging", feature = "deprecated")))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "deprecated")]
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Width: usize,
    #[cfg(feature = "deprecated")]
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Height: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IImageVariableDescriptorPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061630066, data2: 670, data3: 19909, data4: [162, 248, 95, 183, 99, 21, 65, 80] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IInferencingOptionsPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub PreferredDeviceKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelDeviceKindPreview) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreferredDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub SetPreferredDeviceKind: unsafe extern "system" fn(this: *mut *mut Self, value: LearningModelDeviceKindPreview) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPreferredDeviceKind: usize,
    #[cfg(feature = "deprecated")]
    pub IsTracingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsTracingEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsTracingEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsTracingEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MaxBatchSize: usize,
    #[cfg(feature = "deprecated")]
    pub SetMaxBatchSize: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetMaxBatchSize: usize,
    #[cfg(feature = "deprecated")]
    pub MinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    MinimizeMemoryAllocation: usize,
    #[cfg(feature = "deprecated")]
    pub SetMinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetMinimizeMemoryAllocation: usize,
    #[cfg(feature = "deprecated")]
    pub ReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReclaimMemoryAfterEvaluation: usize,
    #[cfg(feature = "deprecated")]
    pub SetReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetReclaimMemoryAfterEvaluation: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IInferencingOptionsPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1203536389, data2: 19766, data3: 18345, data4: [143, 104, 255, 203, 51, 157, 208, 252] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelBindingPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Bind: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Bind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void, metadata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    BindWithProperties: usize,
    #[cfg(feature = "deprecated")]
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Clear: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelBindingPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2479423976, data2: 27768, data3: 19279, data4: [174, 193, 166, 187, 158, 105, 22, 36] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelBindingPreviewFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CreateFromModel: unsafe extern "system" fn(this: *mut *mut Self, model: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateFromModel: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelBindingPreviewFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1220026783, data2: 7761, data3: 19831, data4: [174, 80, 62, 193, 100, 173, 52, 128] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelDescriptionPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Author: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Author: usize,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Domain: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub Version: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Version: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Metadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Metadata: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub InputFeatures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    InputFeatures: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    OutputFeatures: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelDescriptionPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4113304006, data2: 34321, data3: 16557, data4: [142, 89, 222, 63, 215, 3, 10, 64] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelEvaluationResultPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub CorrelationId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CorrelationId: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Outputs: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Outputs: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelEvaluationResultPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3743804063, data2: 39011, data3: 16520, data4: [132, 152, 135, 161, 244, 104, 111, 146] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut *mut Self, binding: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    EvaluateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut *mut Self, features: *mut ::core::ffi::c_void, correlationid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    EvaluateFeaturesAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub InferencingOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InferencingOptions: usize,
    #[cfg(feature = "deprecated")]
    pub SetInferencingOptions: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetInferencingOptions: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 77342314, data2: 37812, data3: 18316, data4: [174, 184, 112, 21, 123, 240, 255, 148] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelPreviewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub LoadModelFromStorageFileAsync: unsafe extern "system" fn(this: *mut *mut Self, modelfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    LoadModelFromStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub LoadModelFromStreamAsync: unsafe extern "system" fn(this: *mut *mut Self, modelstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    LoadModelFromStreamAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelPreviewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 374061920, data2: 33893, data3: 18310, data4: [139, 147, 44, 22, 168, 146, 137, 215] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ILearningModelVariableDescriptorPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Name: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub ModelFeatureKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LearningModelFeatureKindPreview) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelFeatureKind: usize,
    #[cfg(feature = "deprecated")]
    pub IsRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsRequired: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ILearningModelVariableDescriptorPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2973628034, data2: 64560, data3: 18731, data4: [142, 160, 237, 31, 83, 192, 176, 56] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IMapVariableDescriptorPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub KeyKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FeatureElementKindPreview) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    KeyKind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ValidStringKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ValidStringKeys: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ValidIntegerKeys: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ValidIntegerKeys: usize,
    #[cfg(feature = "deprecated")]
    pub Fields: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Fields: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IMapVariableDescriptorPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1018397552, data2: 49195, data3: 16950, data4: [179, 232, 107, 220, 164, 156, 49, 41] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISequenceVariableDescriptorPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub ElementType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ElementType: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ISequenceVariableDescriptorPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2631463570, data2: 39090, data3: 17712, data4: [161, 182, 45, 237, 95, 236, 188, 38] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ITensorVariableDescriptorPreview {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub DataType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut FeatureElementKindPreview) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DataType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Shape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Shape: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for ITensorVariableDescriptorPreview {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2819575834, data2: 39596, data3: 16947, data4: [151, 132, 172, 234, 249, 37, 16, 181] };
}
pub type ImageVariableDescriptorPreview = *mut ::core::ffi::c_void;
pub type InferencingOptionsPreview = *mut ::core::ffi::c_void;
pub type LearningModelBindingPreview = *mut ::core::ffi::c_void;
pub type LearningModelDescriptionPreview = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelDeviceKindPreview(pub i32);
#[cfg(feature = "deprecated")]
impl LearningModelDeviceKindPreview {
    pub const LearningDeviceAny: Self = Self(0i32);
    pub const LearningDeviceCpu: Self = Self(1i32);
    pub const LearningDeviceGpu: Self = Self(2i32);
    pub const LearningDeviceNpu: Self = Self(3i32);
    pub const LearningDeviceDsp: Self = Self(4i32);
    pub const LearningDeviceFpga: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for LearningModelDeviceKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelDeviceKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LearningModelEvaluationResultPreview = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"AI_MachineLearning_Preview\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct LearningModelFeatureKindPreview(pub i32);
#[cfg(feature = "deprecated")]
impl LearningModelFeatureKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Tensor: Self = Self(1i32);
    pub const Sequence: Self = Self(2i32);
    pub const Map: Self = Self(3i32);
    pub const Image: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for LearningModelFeatureKindPreview {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for LearningModelFeatureKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LearningModelPreview = *mut ::core::ffi::c_void;
pub type LearningModelVariableDescriptorPreview = *mut ::core::ffi::c_void;
pub type MapVariableDescriptorPreview = *mut ::core::ffi::c_void;
pub type SequenceVariableDescriptorPreview = *mut ::core::ffi::c_void;
pub type TensorVariableDescriptorPreview = *mut ::core::ffi::c_void;
