#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FeatureElementKindPreview(pub i32);
#[cfg(feature = "winrt-")]
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
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for FeatureElementKindPreview {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for FeatureElementKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for FeatureElementKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for FeatureElementKindPreview {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for FeatureElementKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FeatureElementKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for FeatureElementKindPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.FeatureElementKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IImageVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ae1fa72_029e_4dc5_a2f8_5fb763154150);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IImageVariableDescriptorPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::Imaging::BitmapPixelFormat) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-graphics", feature = "winrt-")))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "winrt-")]
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Width: usize,
    #[cfg(feature = "winrt-")]
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Height: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IInferencingOptionsPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IInferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47bc8205_4d36_47a9_8f68_ffcb339dd0fc);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IInferencingOptionsPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub PreferredDeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelDeviceKindPreview) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    PreferredDeviceKind: usize,
    #[cfg(feature = "winrt-")]
    pub SetPreferredDeviceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LearningModelDeviceKindPreview) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetPreferredDeviceKind: usize,
    #[cfg(feature = "winrt-")]
    pub IsTracingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsTracingEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub SetIsTracingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetIsTracingEnabled: usize,
    #[cfg(feature = "winrt-")]
    pub MaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MaxBatchSize: usize,
    #[cfg(feature = "winrt-")]
    pub SetMaxBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetMaxBatchSize: usize,
    #[cfg(feature = "winrt-")]
    pub MinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    MinimizeMemoryAllocation: usize,
    #[cfg(feature = "winrt-")]
    pub SetMinimizeMemoryAllocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetMinimizeMemoryAllocation: usize,
    #[cfg(feature = "winrt-")]
    pub ReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ReclaimMemoryAfterEvaluation: usize,
    #[cfg(feature = "winrt-")]
    pub SetReclaimMemoryAfterEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetReclaimMemoryAfterEvaluation: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelBindingPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93c901e8_6c78_4b4f_aec1_a6bb9e691624);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Bind: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub BindWithProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void, metadata: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    BindWithProperties: usize,
    #[cfg(feature = "winrt-")]
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Clear: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelBindingPreviewFactory(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelBindingPreviewFactory {
    type Vtable = ILearningModelBindingPreviewFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48b8219f_1e51_4d77_ae50_3ec164ad3480);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelBindingPreviewFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CreateFromModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, model: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CreateFromModel: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelDescriptionPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf52c09c6_8611_40ad_8e59_de3fd7030a40);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDescriptionPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Author: usize,
    #[cfg(feature = "winrt-")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Name: usize,
    #[cfg(feature = "winrt-")]
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Domain: usize,
    #[cfg(feature = "winrt-")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Description: usize,
    #[cfg(feature = "winrt-")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Version: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Metadata: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub InputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    InputFeatures: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub OutputFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    OutputFeatures: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelEvaluationResultPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf25ea9f_9863_4088_8498_87a1f4686f92);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelEvaluationResultPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    CorrelationId: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Outputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Outputs: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelPreview {
    type Vtable = ILearningModelPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x049c266a_93b4_478c_aeb8_70157bf0ff94);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub EvaluateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: ::windows_core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    EvaluateAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: ::windows_core::RawPtr, correlationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    EvaluateFeaturesAsync: usize,
    #[cfg(feature = "winrt-")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Description: usize,
    #[cfg(feature = "winrt-")]
    pub InferencingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    InferencingOptions: usize,
    #[cfg(feature = "winrt-")]
    pub SetInferencingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    SetInferencingOptions: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelPreviewStatics(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelPreviewStatics {
    type Vtable = ILearningModelPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x164bbb60_8465_4786_8b93_2c16a89289d7);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub LoadModelFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelfile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    LoadModelFromStorageFileAsync: usize,
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub LoadModelFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modelstream: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-storage", feature = "winrt-")))]
    LoadModelFromStreamAsync: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ILearningModelVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ILearningModelVariableDescriptorPreview {
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ILearningModelVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: ILearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ILearningModelVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &ILearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ILearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ILearningModelVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ILearningModelVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ILearningModelVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILearningModelVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ILearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b13df682-fc30-492b-8ea0-ed1f53c0b038}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ILearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb13df682_fc30_492b_8ea0_ed1f53c0b038);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelVariableDescriptorPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Name: usize,
    #[cfg(feature = "winrt-")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Description: usize,
    #[cfg(feature = "winrt-")]
    pub ModelFeatureKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LearningModelFeatureKindPreview) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ModelFeatureKind: usize,
    #[cfg(feature = "winrt-")]
    pub IsRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    IsRequired: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct IMapVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for IMapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cb38370_c02b_4236_b3e8_6bdca49c3129);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct IMapVariableDescriptorPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub KeyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    KeyKind: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub ValidStringKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    ValidStringKeys: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub ValidIntegerKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    ValidIntegerKeys: usize,
    #[cfg(feature = "winrt-")]
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    Fields: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ISequenceVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ISequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9cd8f292_98b2_4530_a1b6_2ded5fecbc26);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ISequenceVariableDescriptorPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub ElementType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    ElementType: usize,
}
#[doc(hidden)]
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ITensorVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ITensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa80f501a_9aac_4233_9784_aceaf92510b5);
}
#[cfg(feature = "winrt-")]
#[repr(C)]
#[doc(hidden)]
pub struct ITensorVariableDescriptorPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-")]
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FeatureElementKindPreview) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-"))]
    DataType: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-")))]
    Shape: usize,
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct ImageVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl ImageVariableDescriptorPreview {
    #[cfg(all(feature = "winrt-graphics", feature = "winrt-"))]
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<::winrt_graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::Imaging::BitmapPixelFormat>::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for ImageVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for ImageVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for ImageVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for ImageVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for ImageVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview;{7ae1fa72-029e-4dc5-a2f8-5fb763154150})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for ImageVariableDescriptorPreview {
    type Vtable = IImageVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = <IImageVariableDescriptorPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for ImageVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.ImageVariableDescriptorPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<ImageVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: ImageVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&ImageVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &ImageVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: ImageVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&ImageVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: &ImageVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &ImageVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct InferencingOptionsPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl InferencingOptionsPreview {
    #[cfg(feature = "winrt-")]
    pub fn PreferredDeviceKind(&self) -> ::windows_core::Result<LearningModelDeviceKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelDeviceKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).PreferredDeviceKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelDeviceKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetPreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredDeviceKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsTracingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsTracingEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetIsTracingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTracingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn MaxBatchSize(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxBatchSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetMaxBatchSize(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxBatchSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn MinimizeMemoryAllocation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).MinimizeMemoryAllocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetMinimizeMemoryAllocation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinimizeMemoryAllocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ReclaimMemoryAfterEvaluation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ReclaimMemoryAfterEvaluation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetReclaimMemoryAfterEvaluation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReclaimMemoryAfterEvaluation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for InferencingOptionsPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for InferencingOptionsPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for InferencingOptionsPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for InferencingOptionsPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InferencingOptionsPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for InferencingOptionsPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.InferencingOptionsPreview;{47bc8205-4d36-47a9-8f68-ffcb339dd0fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for InferencingOptionsPreview {
    type Vtable = IInferencingOptionsPreview_Vtbl;
    const IID: ::windows_core::GUID = <IInferencingOptionsPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for InferencingOptionsPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.InferencingOptionsPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<InferencingOptionsPreview> for ::windows_core::IUnknown {
    fn from(value: InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows_core::IUnknown {
    fn from(value: &InferencingOptionsPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InferencingOptionsPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InferencingOptionsPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<InferencingOptionsPreview> for ::windows_core::IInspectable {
    fn from(value: InferencingOptionsPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&InferencingOptionsPreview> for ::windows_core::IInspectable {
    fn from(value: &InferencingOptionsPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InferencingOptionsPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InferencingOptionsPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct LearningModelBindingPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl LearningModelBindingPreview {
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Bind<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Bind)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn BindWithProperties<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IPropertySet>>(&self, name: Param0, value: Param1, metadata: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BindWithProperties)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi(), metadata.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn CreateFromModel<'a, Param0: ::windows_core::IntoParam<'a, LearningModelPreview>>(model: Param0) -> ::windows_core::Result<LearningModelBindingPreview> {
        Self::ILearningModelBindingPreviewFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModel)(::windows_core::Interface::as_raw(this), model.into_param().abi(), result__.as_mut_ptr()).from_abi::<LearningModelBindingPreview>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Split(&self, first: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>, second: &mut ::core::option::Option<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[cfg(feature = "winrt-")]
    pub fn ILearningModelBindingPreviewFactory<R, F: FnOnce(&ILearningModelBindingPreviewFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LearningModelBindingPreview, ILearningModelBindingPreviewFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelBindingPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for LearningModelBindingPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for LearningModelBindingPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelBindingPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelBindingPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelBindingPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelBindingPreview;{93c901e8-6c78-4b4f-aec1-a6bb9e691624})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for LearningModelBindingPreview {
    type Vtable = ILearningModelBindingPreview_Vtbl;
    const IID: ::windows_core::GUID = <ILearningModelBindingPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for LearningModelBindingPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelBindingPreview";
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::iter::IntoIterator for LearningModelBindingPreview {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::iter::IntoIterator for &LearningModelBindingPreview {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelBindingPreview> for ::windows_core::IUnknown {
    fn from(value: LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows_core::IUnknown {
    fn from(value: &LearningModelBindingPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelBindingPreview> for ::windows_core::IInspectable {
    fn from(value: LearningModelBindingPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelBindingPreview> for ::windows_core::IInspectable {
    fn from(value: &LearningModelBindingPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for &LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<LearningModelBindingPreview> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: LearningModelBindingPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl ::core::convert::TryFrom<&LearningModelBindingPreview> for ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable> {
    type Error = ::windows_core::Error;
    fn try_from(value: &LearningModelBindingPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> for &LearningModelBindingPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct LearningModelDescriptionPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl LearningModelDescriptionPreview {
    #[cfg(feature = "winrt-")]
    pub fn Author(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Domain(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Domain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Version(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows_core::Interface::vtable(this).Version)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Metadata(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Metadata)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn InputFeatures(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputFeatures)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn OutputFeatures(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OutputFeatures)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<ILearningModelVariableDescriptorPreview>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelDescriptionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for LearningModelDescriptionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for LearningModelDescriptionPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelDescriptionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDescriptionPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelDescriptionPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview;{f52c09c6-8611-40ad-8e59-de3fd7030a40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for LearningModelDescriptionPreview {
    type Vtable = ILearningModelDescriptionPreview_Vtbl;
    const IID: ::windows_core::GUID = <ILearningModelDescriptionPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for LearningModelDescriptionPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelDescriptionPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows_core::IUnknown {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows_core::IUnknown {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelDescriptionPreview> for ::windows_core::IInspectable {
    fn from(value: LearningModelDescriptionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelDescriptionPreview> for ::windows_core::IInspectable {
    fn from(value: &LearningModelDescriptionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LearningModelDescriptionPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LearningModelDeviceKindPreview(pub i32);
#[cfg(feature = "winrt-")]
impl LearningModelDeviceKindPreview {
    pub const LearningDeviceAny: Self = Self(0i32);
    pub const LearningDeviceCpu: Self = Self(1i32);
    pub const LearningDeviceGpu: Self = Self(2i32);
    pub const LearningDeviceNpu: Self = Self(3i32);
    pub const LearningDeviceDsp: Self = Self(4i32);
    pub const LearningDeviceFpga: Self = Self(5i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for LearningModelDeviceKindPreview {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelDeviceKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for LearningModelDeviceKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for LearningModelDeviceKindPreview {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelDeviceKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelDeviceKindPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelDeviceKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct LearningModelEvaluationResultPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl LearningModelEvaluationResultPreview {
    #[cfg(feature = "winrt-")]
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Outputs(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Outputs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelEvaluationResultPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for LearningModelEvaluationResultPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for LearningModelEvaluationResultPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelEvaluationResultPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelEvaluationResultPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelEvaluationResultPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview;{df25ea9f-9863-4088-8498-87a1f4686f92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for LearningModelEvaluationResultPreview {
    type Vtable = ILearningModelEvaluationResultPreview_Vtbl;
    const IID: ::windows_core::GUID = <ILearningModelEvaluationResultPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for LearningModelEvaluationResultPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelEvaluationResultPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows_core::IUnknown {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows_core::IUnknown {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelEvaluationResultPreview> for ::windows_core::IInspectable {
    fn from(value: LearningModelEvaluationResultPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelEvaluationResultPreview> for ::windows_core::IInspectable {
    fn from(value: &LearningModelEvaluationResultPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LearningModelEvaluationResultPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LearningModelFeatureKindPreview(pub i32);
#[cfg(feature = "winrt-")]
impl LearningModelFeatureKindPreview {
    pub const Undefined: Self = Self(0i32);
    pub const Tensor: Self = Self(1i32);
    pub const Sequence: Self = Self(2i32);
    pub const Map: Self = Self(3i32);
    pub const Image: Self = Self(4i32);
}
#[cfg(feature = "winrt-")]
impl ::core::marker::Copy for LearningModelFeatureKindPreview {}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelFeatureKindPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "winrt-")]
impl ::core::default::Default for LearningModelFeatureKindPreview {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Abi for LearningModelFeatureKindPreview {
    type Abi = Self;
}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelFeatureKindPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKindPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelFeatureKindPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.Preview.LearningModelFeatureKindPreview;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct LearningModelPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl LearningModelPreview {
    #[cfg(feature = "winrt-")]
    pub fn EvaluateAsync<'a, Param0: ::windows_core::IntoParam<'a, LearningModelBindingPreview>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, binding: Param0, correlationid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EvaluateAsync)(::windows_core::Interface::as_raw(this), binding.into_param().abi(), correlationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn EvaluateFeaturesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, features: Param0, correlationid: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EvaluateFeaturesAsync)(::windows_core::Interface::as_raw(this), features.into_param().abi(), correlationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<LearningModelDescriptionPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelDescriptionPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn InferencingOptions(&self) -> ::windows_core::Result<InferencingOptionsPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InferencingOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InferencingOptionsPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn SetInferencingOptions<'a, Param0: ::windows_core::IntoParam<'a, InferencingOptionsPreview>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInferencingOptions)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn LoadModelFromStorageFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::IStorageFile>>(modelfile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LearningModelPreview>> {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadModelFromStorageFileAsync)(::windows_core::Interface::as_raw(this), modelfile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-storage", feature = "winrt-"))]
    pub fn LoadModelFromStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStreamReference>>(modelstream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<LearningModelPreview>> {
        Self::ILearningModelPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadModelFromStreamAsync)(::windows_core::Interface::as_raw(this), modelstream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<LearningModelPreview>>(result__)
        })
    }
    #[cfg(feature = "winrt-")]
    pub fn ILearningModelPreviewStatics<R, F: FnOnce(&ILearningModelPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LearningModelPreview, ILearningModelPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for LearningModelPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for LearningModelPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelPreview;{049c266a-93b4-478c-aeb8-70157bf0ff94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for LearningModelPreview {
    type Vtable = ILearningModelPreview_Vtbl;
    const IID: ::windows_core::GUID = <ILearningModelPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for LearningModelPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelPreview> for ::windows_core::IUnknown {
    fn from(value: LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelPreview> for ::windows_core::IUnknown {
    fn from(value: &LearningModelPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LearningModelPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LearningModelPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelPreview> for ::windows_core::IInspectable {
    fn from(value: LearningModelPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelPreview> for ::windows_core::IInspectable {
    fn from(value: &LearningModelPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LearningModelPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LearningModelPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct LearningModelVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl LearningModelVariableDescriptorPreview {
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for LearningModelVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for LearningModelVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for LearningModelVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for LearningModelVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LearningModelVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for LearningModelVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview;{b13df682-fc30-492b-8ea0-ed1f53c0b038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for LearningModelVariableDescriptorPreview {
    type Vtable = ILearningModelVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = <ILearningModelVariableDescriptorPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for LearningModelVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.LearningModelVariableDescriptorPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<LearningModelVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: LearningModelVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&LearningModelVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &LearningModelVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: LearningModelVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&LearningModelVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: &LearningModelVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &LearningModelVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct MapVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl MapVariableDescriptorPreview {
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn KeyKind(&self) -> ::windows_core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FeatureElementKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).KeyKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn ValidStringKeys(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValidStringKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn ValidIntegerKeys(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValidIntegerKeys)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<i64>>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Fields(&self) -> ::windows_core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Fields)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for MapVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for MapVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for MapVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for MapVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for MapVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview;{3cb38370-c02b-4236-b3e8-6bdca49c3129})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for MapVariableDescriptorPreview {
    type Vtable = IMapVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = <IMapVariableDescriptorPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for MapVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.MapVariableDescriptorPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<MapVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: MapVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&MapVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &MapVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: MapVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&MapVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: &MapVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &MapVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct SequenceVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl SequenceVariableDescriptorPreview {
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ElementType(&self) -> ::windows_core::Result<ILearningModelVariableDescriptorPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ILearningModelVariableDescriptorPreview>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for SequenceVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for SequenceVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for SequenceVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for SequenceVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SequenceVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for SequenceVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview;{9cd8f292-98b2-4530-a1b6-2ded5fecbc26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for SequenceVariableDescriptorPreview {
    type Vtable = ISequenceVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = <ISequenceVariableDescriptorPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for SequenceVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.SequenceVariableDescriptorPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<SequenceVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: SequenceVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&SequenceVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &SequenceVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: SequenceVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&SequenceVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: &SequenceVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &SequenceVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-")]
#[repr(transparent)]
pub struct TensorVariableDescriptorPreview(::windows_core::IUnknown);
#[cfg(feature = "winrt-")]
impl TensorVariableDescriptorPreview {
    #[cfg(feature = "winrt-")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn ModelFeatureKind(&self) -> ::windows_core::Result<LearningModelFeatureKindPreview> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<LearningModelFeatureKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).ModelFeatureKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LearningModelFeatureKindPreview>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn IsRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ILearningModelVariableDescriptorPreview>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequired)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-")]
    pub fn DataType(&self) -> ::windows_core::Result<FeatureElementKindPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FeatureElementKindPreview>::zeroed();
            (::windows_core::Interface::vtable(this).DataType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FeatureElementKindPreview>(result__)
        }
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-"))]
    pub fn Shape(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterable<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterable<i64>>(result__)
        }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::clone::Clone for TensorVariableDescriptorPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::PartialEq for TensorVariableDescriptorPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-")]
impl ::core::cmp::Eq for TensorVariableDescriptorPreview {}
#[cfg(feature = "winrt-")]
impl ::core::fmt::Debug for TensorVariableDescriptorPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TensorVariableDescriptorPreview").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::RuntimeType for TensorVariableDescriptorPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview;{a80f501a-9aac-4233-9784-aceaf92510b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-")]
unsafe impl ::windows_core::Interface for TensorVariableDescriptorPreview {
    type Vtable = ITensorVariableDescriptorPreview_Vtbl;
    const IID: ::windows_core::GUID = <ITensorVariableDescriptorPreview as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-")]
impl ::windows_core::RuntimeName for TensorVariableDescriptorPreview {
    const NAME: &'static str = "Windows.AI.MachineLearning.Preview.TensorVariableDescriptorPreview";
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows_core::IUnknown {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<TensorVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: TensorVariableDescriptorPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::From<&TensorVariableDescriptorPreview> for ::windows_core::IInspectable {
    fn from(value: &TensorVariableDescriptorPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: TensorVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-")]
impl ::core::convert::TryFrom<&TensorVariableDescriptorPreview> for ILearningModelVariableDescriptorPreview {
    type Error = ::windows_core::Error;
    fn try_from(value: &TensorVariableDescriptorPreview) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-")]
impl<'a> ::windows_core::IntoParam<'a, ILearningModelVariableDescriptorPreview> for &TensorVariableDescriptorPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ILearningModelVariableDescriptorPreview> {
        ::core::convert::TryInto::<ILearningModelVariableDescriptorPreview>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
