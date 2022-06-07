#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
    pub fn MLCreateOperatorRegistry(registry: *mut *mut *mut IMLOperatorRegistry) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
    pub fn WinMLCreateRuntime(runtime: *mut *mut *mut IWinMLRuntime) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct IMLOperatorAttributes {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetAttributeElementCount: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCSTR, r#type: MLOperatorAttributeType, elementcount: u32, elementbytesize: usize, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStringAttributeElementLength: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCSTR, elementindex: u32, attributeelementbytesize: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetStringAttributeElement: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCSTR, elementindex: u32, attributeelementbytesize: u32, attributeelement: ::windows_sys::core::PSTR) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1260066649, data2: 60480, data3: 18028, data4: [170, 180, 190, 181, 52, 127, 210, 76] };
}
#[repr(C)]
pub struct IMLOperatorKernel {
    pub base__: ::windows_sys::core::IUnknown,
    pub Compute: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorKernel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 298103968, data2: 46183, data3: 20138, data4: [161, 166, 185, 97, 216, 208, 237, 121] };
}
#[repr(C)]
pub struct IMLOperatorKernelContext {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputTensor: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputTensor: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, dimensioncount: u32, dimensionsizes: *const u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetOutputTensor2: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, tensor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AllocateTemporaryData: unsafe extern "system" fn(this: *mut *mut Self, size: usize, data: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(this: *mut *mut Self, executionobject: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for IMLOperatorKernelContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2186504744, data2: 61474, data3: 18281, data4: [157, 63, 139, 39, 143, 132, 192, 195] };
}
#[repr(C)]
pub struct IMLOperatorKernelCreationContext {
    pub base__: IMLOperatorAttributes,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_sys::core::HRESULT,
    pub GetOutputEdgeDescription: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_sys::core::HRESULT,
    pub HasTensorShapeDescription: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub GetTensorShapeDescription: unsafe extern "system" fn(this: *mut *mut Self, shapedescription: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetExecutionInterface: unsafe extern "system" fn(this: *mut *mut Self, executionobject: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for IMLOperatorKernelCreationContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1415165245, data2: 41212, data3: 18021, data4: [173, 221, 112, 23, 30, 247, 230, 49] };
}
#[repr(C)]
pub struct IMLOperatorKernelFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateKernel: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void, kernel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorKernelFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4011175279, data2: 3529, data3: 18696, data4: [171, 53, 165, 117, 163, 13, 251, 248] };
}
#[repr(C)]
pub struct IMLOperatorRegistry {
    pub base__: ::windows_sys::core::IUnknown,
    pub RegisterOperatorSetSchema: unsafe extern "system" fn(this: *mut *mut Self, operatorsetid: *const MLOperatorSetId, baselineversion: i32, schema: *const *const MLOperatorSchemaDescription, schemacount: u32, typeinferrer: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RegisterOperatorKernel: unsafe extern "system" fn(this: *mut *mut Self, operatorkernel: *const MLOperatorKernelDescription, operatorkernelfactory: *mut ::core::ffi::c_void, shapeinferrer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorRegistry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 721018157, data2: 46358, data3: 18034, data4: [154, 181, 83, 12, 32, 132, 147, 173] };
}
#[repr(C)]
pub struct IMLOperatorShapeInferenceContext {
    pub base__: IMLOperatorAttributes,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_sys::core::HRESULT,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, dimensioncount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetOutputTensorShape: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, dimensioncount: u32, dimensions: *const u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorShapeInferenceContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 274426665, data2: 21512, data3: 19048, data4: [153, 89, 9, 181, 149, 90, 52, 146] };
}
#[repr(C)]
pub struct IMLOperatorShapeInferrer {
    pub base__: ::windows_sys::core::IUnknown,
    pub InferOutputShapes: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorShapeInferrer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1410065854, data2: 42697, data3: 16622, data4: [131, 246, 210, 184, 180, 10, 119, 152] };
}
#[repr(C)]
pub struct IMLOperatorTensor {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDimensionCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetShape: unsafe extern "system" fn(this: *mut *mut Self, dimensioncount: u32, dimensions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetTensorDataType: unsafe extern "system" fn(this: *mut *mut Self) -> MLOperatorTensorDataType,
    pub IsCpuData: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub IsDataInterface: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub GetData: unsafe extern "system" fn(this: *mut *mut Self) -> *mut ::core::ffi::c_void,
    pub GetDataInterface: unsafe extern "system" fn(this: *mut *mut Self, datainterface: *mut *mut ::core::ffi::c_void),
}
impl ::windows_sys::core::Interface for IMLOperatorTensor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2145656641, data2: 62512, data3: 17422, data4: [174, 206, 84, 65, 109, 200, 185, 219] };
}
#[repr(C)]
pub struct IMLOperatorTensorShapeDescription {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetInputTensorDimensionCount: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, dimensioncount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInputTensorShape: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HasOutputShapeDescription: unsafe extern "system" fn(this: *mut *mut Self) -> bool,
    pub GetOutputTensorDimensionCount: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, dimensioncount: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetOutputTensorShape: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, dimensioncount: u32, dimensions: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorTensorShapeDescription {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061039806, data2: 15144, data3: 16968, data4: [190, 149, 249, 111, 188, 110, 70, 67] };
}
#[repr(C)]
pub struct IMLOperatorTypeInferenceContext {
    pub base__: IMLOperatorAttributes,
    pub GetInputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut *mut Self) -> u32,
    pub IsInputValid: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32) -> bool,
    pub IsOutputValid: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32) -> bool,
    pub GetInputEdgeDescription: unsafe extern "system" fn(this: *mut *mut Self, inputindex: u32, edgedescription: *mut MLOperatorEdgeDescription) -> ::windows_sys::core::HRESULT,
    pub SetOutputEdgeDescription: unsafe extern "system" fn(this: *mut *mut Self, outputindex: u32, edgedescription: *const MLOperatorEdgeDescription) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorTypeInferenceContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3968416689, data2: 63800, data3: 17019, data4: [132, 136, 200, 220, 247, 117, 241, 56] };
}
#[repr(C)]
pub struct IMLOperatorTypeInferrer {
    pub base__: ::windows_sys::core::IUnknown,
    pub InferOutputTypes: unsafe extern "system" fn(this: *mut *mut Self, context: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMLOperatorTypeInferrer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2015030088, data2: 39883, data3: 18327, data4: [191, 119, 139, 244, 85, 33, 123, 235] };
}
#[repr(C)]
pub struct IWinMLEvaluationContext {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub BindValue: unsafe extern "system" fn(this: *mut *mut Self, pdescriptor: *const WINML_BINDING_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    BindValue: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub GetValueByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::PCWSTR, pdescriptor: *mut *mut WINML_BINDING_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    GetValueByName: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWinMLEvaluationContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2508492702, data2: 22589, data3: 16468, data4: [175, 18, 145, 99, 135, 205, 132, 38] };
}
#[repr(C)]
pub struct IWinMLModel {
    pub base__: ::windows_sys::core::IUnknown,
    pub GetDescription: unsafe extern "system" fn(this: *mut *mut Self, ppdescription: *mut *mut WINML_MODEL_DESC) -> ::windows_sys::core::HRESULT,
    pub EnumerateMetadata: unsafe extern "system" fn(this: *mut *mut Self, index: u32, pkey: *mut ::windows_sys::core::PWSTR, pvalue: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateModelInputs: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppinputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateModelInputs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumerateModelOutputs: unsafe extern "system" fn(this: *mut *mut Self, index: u32, ppoutputdescriptor: *mut *mut WINML_VARIABLE_DESC) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumerateModelOutputs: usize,
}
impl ::windows_sys::core::Interface for IWinMLModel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3807295145, data2: 62239, data3: 16469, data4: [165, 33, 227, 11, 91, 51, 102, 74] };
}
#[repr(C)]
pub struct IWinMLRuntime {
    pub base__: ::windows_sys::core::IUnknown,
    pub LoadModel: unsafe extern "system" fn(this: *mut *mut Self, path: ::windows_sys::core::PCWSTR, ppmodel: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateEvaluationContext: unsafe extern "system" fn(this: *mut *mut Self, device: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateEvaluationContext: usize,
    pub EvaluateModel: unsafe extern "system" fn(this: *mut *mut Self, pcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWinMLRuntime {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2688701225, data2: 16558, data3: 18649, data4: [188, 227, 130, 158, 247, 184, 164, 26] };
}
#[repr(C)]
pub struct IWinMLRuntimeFactory {
    pub base__: ::windows_sys::core::IUnknown,
    pub CreateRuntime: unsafe extern "system" fn(this: *mut *mut Self, runtimetype: WINML_RUNTIME_TYPE, ppruntime: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWinMLRuntimeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2819078221, data2: 19173, data3: 19392, data4: [167, 106, 148, 26, 162, 70, 189, 65] };
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorAttribute {
    pub name: ::windows_sys::core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub required: bool,
}
impl ::core::marker::Copy for MLOperatorAttribute {}
impl ::core::clone::Clone for MLOperatorAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorAttributeNameValue {
    pub name: ::windows_sys::core::PCSTR,
    pub r#type: MLOperatorAttributeType,
    pub valueCount: u32,
    pub Anonymous: MLOperatorAttributeNameValue_0,
}
impl ::core::marker::Copy for MLOperatorAttributeNameValue {}
impl ::core::clone::Clone for MLOperatorAttributeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorAttributeNameValue_0 {
    pub reserved: *const ::core::ffi::c_void,
    pub ints: *const i64,
    pub strings: *const *const i8,
    pub floats: *const f32,
}
impl ::core::marker::Copy for MLOperatorAttributeNameValue_0 {}
impl ::core::clone::Clone for MLOperatorAttributeNameValue_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorAttributeType(pub u32);
impl MLOperatorAttributeType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(2u32);
    pub const Int: Self = Self(3u32);
    pub const String: Self = Self(4u32);
    pub const FloatArray: Self = Self(7u32);
    pub const IntArray: Self = Self(8u32);
    pub const StringArray: Self = Self(9u32);
}
impl ::core::marker::Copy for MLOperatorAttributeType {}
impl ::core::clone::Clone for MLOperatorAttributeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorEdgeDescription {
    pub edgeType: MLOperatorEdgeType,
    pub Anonymous: MLOperatorEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription {}
impl ::core::clone::Clone for MLOperatorEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorEdgeDescription_0 {
    pub reserved: u64,
    pub tensorDataType: MLOperatorTensorDataType,
}
impl ::core::marker::Copy for MLOperatorEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorEdgeType(pub u32);
impl MLOperatorEdgeType {
    pub const Undefined: Self = Self(0u32);
    pub const Tensor: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorEdgeType {}
impl ::core::clone::Clone for MLOperatorEdgeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorEdgeTypeConstraint {
    pub typeLabel: ::windows_sys::core::PCSTR,
    pub allowedTypes: *const MLOperatorEdgeDescription,
    pub allowedTypeCount: u32,
}
impl ::core::marker::Copy for MLOperatorEdgeTypeConstraint {}
impl ::core::clone::Clone for MLOperatorEdgeTypeConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorExecutionType(pub u32);
impl MLOperatorExecutionType {
    pub const Undefined: Self = Self(0u32);
    pub const Cpu: Self = Self(1u32);
    pub const D3D12: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorExecutionType {}
impl ::core::clone::Clone for MLOperatorExecutionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorKernelDescription {
    pub domain: ::windows_sys::core::PCSTR,
    pub name: ::windows_sys::core::PCSTR,
    pub minimumOperatorSetVersion: i32,
    pub executionType: MLOperatorExecutionType,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
    pub options: MLOperatorKernelOptions,
    pub executionOptions: u32,
}
impl ::core::marker::Copy for MLOperatorKernelDescription {}
impl ::core::clone::Clone for MLOperatorKernelDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorKernelOptions(pub u32);
impl MLOperatorKernelOptions {
    pub const None: Self = Self(0u32);
    pub const AllowDynamicInputShapes: Self = Self(1u32);
}
impl ::core::marker::Copy for MLOperatorKernelOptions {}
impl ::core::clone::Clone for MLOperatorKernelOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorParameterOptions(pub u32);
impl MLOperatorParameterOptions {
    pub const Single: Self = Self(0u32);
    pub const Optional: Self = Self(1u32);
    pub const Variadic: Self = Self(2u32);
}
impl ::core::marker::Copy for MLOperatorParameterOptions {}
impl ::core::clone::Clone for MLOperatorParameterOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSchemaDescription {
    pub name: ::windows_sys::core::PCSTR,
    pub operatorSetVersionAtLastChange: i32,
    pub inputs: *const MLOperatorSchemaEdgeDescription,
    pub inputCount: u32,
    pub outputs: *const MLOperatorSchemaEdgeDescription,
    pub outputCount: u32,
    pub typeConstraints: *const MLOperatorEdgeTypeConstraint,
    pub typeConstraintCount: u32,
    pub attributes: *const MLOperatorAttribute,
    pub attributeCount: u32,
    pub defaultAttributes: *const MLOperatorAttributeNameValue,
    pub defaultAttributeCount: u32,
}
impl ::core::marker::Copy for MLOperatorSchemaDescription {}
impl ::core::clone::Clone for MLOperatorSchemaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSchemaEdgeDescription {
    pub options: MLOperatorParameterOptions,
    pub typeFormat: MLOperatorSchemaEdgeTypeFormat,
    pub Anonymous: MLOperatorSchemaEdgeDescription_0,
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union MLOperatorSchemaEdgeDescription_0 {
    pub reserved: *const ::core::ffi::c_void,
    pub typeLabel: ::windows_sys::core::PCSTR,
    pub edgeDescription: MLOperatorEdgeDescription,
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeDescription_0 {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeDescription_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorSchemaEdgeTypeFormat(pub i32);
impl MLOperatorSchemaEdgeTypeFormat {
    pub const EdgeDescription: Self = Self(0i32);
    pub const Label: Self = Self(1i32);
}
impl ::core::marker::Copy for MLOperatorSchemaEdgeTypeFormat {}
impl ::core::clone::Clone for MLOperatorSchemaEdgeTypeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct MLOperatorSetId {
    pub domain: ::windows_sys::core::PCSTR,
    pub version: i32,
}
impl ::core::marker::Copy for MLOperatorSetId {}
impl ::core::clone::Clone for MLOperatorSetId {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
#[repr(transparent)]
pub struct MLOperatorTensorDataType(pub u32);
impl MLOperatorTensorDataType {
    pub const Undefined: Self = Self(0u32);
    pub const Float: Self = Self(1u32);
    pub const UInt8: Self = Self(2u32);
    pub const Int8: Self = Self(3u32);
    pub const UInt16: Self = Self(4u32);
    pub const Int16: Self = Self(5u32);
    pub const Int32: Self = Self(6u32);
    pub const Int64: Self = Self(7u32);
    pub const String: Self = Self(8u32);
    pub const Bool: Self = Self(9u32);
    pub const Float16: Self = Self(10u32);
    pub const Double: Self = Self(11u32);
    pub const UInt32: Self = Self(12u32);
    pub const UInt64: Self = Self(13u32);
    pub const Complex64: Self = Self(14u32);
    pub const Complex128: Self = Self(15u32);
}
impl ::core::marker::Copy for MLOperatorTensorDataType {}
impl ::core::clone::Clone for MLOperatorTensorDataType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_BINDING_DESC {
    pub Name: ::windows_sys::core::PCWSTR,
    pub BindType: WINML_BINDING_TYPE,
    pub Anonymous: WINML_BINDING_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for WINML_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub union WINML_BINDING_DESC_0 {
    pub Tensor: WINML_TENSOR_BINDING_DESC,
    pub Sequence: WINML_SEQUENCE_BINDING_DESC,
    pub Map: WINML_MAP_BINDING_DESC,
    pub Image: WINML_IMAGE_BINDING_DESC,
    pub Resource: WINML_RESOURCE_BINDING_DESC,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for WINML_BINDING_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub type WINML_BINDING_TYPE = i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_UNDEFINED: WINML_BINDING_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_TENSOR: WINML_BINDING_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_SEQUENCE: WINML_BINDING_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_MAP: WINML_BINDING_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_IMAGE: WINML_BINDING_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_BINDING_RESOURCE: WINML_BINDING_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub type WINML_FEATURE_TYPE = i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_UNDEFINED: WINML_FEATURE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_TENSOR: WINML_FEATURE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_SEQUENCE: WINML_FEATURE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_MAP: WINML_FEATURE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_FEATURE_IMAGE: WINML_FEATURE_TYPE = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_IMAGE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_IMAGE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_IMAGE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_IMAGE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_IMAGE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MAP_BINDING_DESC {
    pub ElementCount: u32,
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous1: WINML_MAP_BINDING_DESC_0,
    pub Fields: WINML_TENSOR_DATA_TYPE,
    pub Anonymous2: WINML_MAP_BINDING_DESC_1,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_MAP_BINDING_DESC_0 {
    pub pStringKeys: *mut ::windows_sys::core::PWSTR,
    pub pIntKeys: *mut i64,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_0 {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_MAP_BINDING_DESC_1 {
    pub pStringFields: *mut ::windows_sys::core::PWSTR,
    pub pIntFields: *mut i64,
    pub pFloatFields: *mut f32,
    pub pDoubleFields: *mut f64,
}
impl ::core::marker::Copy for WINML_MAP_BINDING_DESC_1 {}
impl ::core::clone::Clone for WINML_MAP_BINDING_DESC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MAP_VARIABLE_DESC {
    pub KeyType: WINML_TENSOR_DATA_TYPE,
    pub Fields: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_MAP_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_MAP_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_MODEL_DESC {
    pub Author: ::windows_sys::core::PWSTR,
    pub Name: ::windows_sys::core::PWSTR,
    pub Domain: ::windows_sys::core::PWSTR,
    pub Description: ::windows_sys::core::PWSTR,
    pub Version: usize,
}
impl ::core::marker::Copy for WINML_MODEL_DESC {}
impl ::core::clone::Clone for WINML_MODEL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct WINML_RESOURCE_BINDING_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub pResource: *mut *mut *mut *mut super::super::super::Graphics::Direct3D12::ID3D12Resource,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for WINML_RESOURCE_BINDING_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for WINML_RESOURCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub type WINML_RUNTIME_TYPE = i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_RUNTIME_CNTK: WINML_RUNTIME_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_SEQUENCE_BINDING_DESC {
    pub ElementCount: u32,
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub Anonymous: WINML_SEQUENCE_BINDING_DESC_0,
}
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub union WINML_SEQUENCE_BINDING_DESC_0 {
    pub pStrings: *mut ::windows_sys::core::PWSTR,
    pub pInts: *mut i64,
    pub pFloats: *mut f32,
    pub pDoubles: *mut f64,
}
impl ::core::marker::Copy for WINML_SEQUENCE_BINDING_DESC_0 {}
impl ::core::clone::Clone for WINML_SEQUENCE_BINDING_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_SEQUENCE_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for WINML_SEQUENCE_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_SEQUENCE_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_TENSOR_BINDING_DESC {
    pub DataType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
    pub DataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WINML_TENSOR_BINDING_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub type WINML_TENSOR_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UNDEFINED: WINML_TENSOR_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_FLOAT: WINML_TENSOR_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT8: WINML_TENSOR_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT8: WINML_TENSOR_DATA_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT16: WINML_TENSOR_DATA_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT16: WINML_TENSOR_DATA_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT32: WINML_TENSOR_DATA_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_INT64: WINML_TENSOR_DATA_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_STRING: WINML_TENSOR_DATA_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_BOOLEAN: WINML_TENSOR_DATA_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_FLOAT16: WINML_TENSOR_DATA_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_DOUBLE: WINML_TENSOR_DATA_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT32: WINML_TENSOR_DATA_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_UINT64: WINML_TENSOR_DATA_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_COMPLEX64: WINML_TENSOR_DATA_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_COMPLEX128: WINML_TENSOR_DATA_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub const WINML_TENSOR_DIMENSION_COUNT_MAX: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`*"]
pub struct WINML_TENSOR_VARIABLE_DESC {
    pub ElementType: WINML_TENSOR_DATA_TYPE,
    pub NumDimensions: u32,
    pub pShape: *mut i64,
}
impl ::core::marker::Copy for WINML_TENSOR_VARIABLE_DESC {}
impl ::core::clone::Clone for WINML_TENSOR_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WINML_VARIABLE_DESC {
    pub Name: ::windows_sys::core::PWSTR,
    pub Description: ::windows_sys::core::PWSTR,
    pub FeatureType: WINML_FEATURE_TYPE,
    pub Required: super::super::super::Foundation::BOOL,
    pub Anonymous: WINML_VARIABLE_DESC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_WinML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WINML_VARIABLE_DESC_0 {
    pub Tensor: WINML_TENSOR_VARIABLE_DESC,
    pub Sequence: WINML_SEQUENCE_VARIABLE_DESC,
    pub Map: WINML_MAP_VARIABLE_DESC,
    pub Image: WINML_IMAGE_VARIABLE_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINML_VARIABLE_DESC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINML_VARIABLE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
