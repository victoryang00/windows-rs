#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for HandwritingLineHeight {}
impl ::core::clone::Clone for HandwritingLineHeight {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IInkDrawingAttributes {
    pub base__: ::windows_sys::core::IInspectable,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub PenTip: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PenTipShape) -> ::windows_sys::core::HRESULT,
    pub SetPenTip: unsafe extern "system" fn(this: *mut *mut Self, value: PenTipShape) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub SetSize: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Size) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSize: usize,
    pub IgnorePressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnorePressure: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub FitToCurve: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetFitToCurve: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2543982444, data2: 26484, data3: 18605, data4: [132, 240, 72, 245, 169, 190, 116, 249] };
}
#[repr(C)]
pub struct IInkDrawingAttributes2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub PenTipTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PenTipTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPenTipTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPenTipTransform: usize,
    pub DrawAsHighlighter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetDrawAsHighlighter: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributes2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2091607304, data2: 36548, data3: 17149, data4: [165, 165, 228, 183, 209, 213, 49, 109] };
}
#[repr(C)]
pub struct IInkDrawingAttributes3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkDrawingAttributesKind) -> ::windows_sys::core::HRESULT,
    pub PencilProperties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributes3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1912733698, data2: 32091, data3: 18064, data4: [138, 244, 230, 100, 203, 226, 183, 79] };
}
#[repr(C)]
pub struct IInkDrawingAttributes4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IgnoreTilt: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIgnoreTilt: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributes4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016430117, data2: 40729, data3: 17773, data4: [145, 163, 188, 58, 61, 145, 197, 251] };
}
#[repr(C)]
pub struct IInkDrawingAttributes5 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ModelerAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributes5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3508183227, data2: 1909, data3: 18514, data4: [174, 100, 65, 20, 58, 122, 230, 201] };
}
#[repr(C)]
pub struct IInkDrawingAttributesPencilProperties {
    pub base__: ::windows_sys::core::IInspectable,
    pub Opacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributesPencilProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1327838411, data2: 11654, data3: 16827, data4: [176, 232, 228, 194, 160, 37, 60, 82] };
}
#[repr(C)]
pub struct IInkDrawingAttributesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateForPencil: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkDrawingAttributesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4147241023, data2: 6757, data3: 18530, data4: [150, 203, 110, 22, 101, 225, 127, 109] };
}
#[repr(C)]
pub struct IInkInputConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPrimaryBarrelButtonInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsEraserInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsEraserInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkInputConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2477166020, data2: 2939, data3: 18903, data4: [179, 79, 153, 1, 229, 36, 220, 242] };
}
#[repr(C)]
pub struct IInkInputConfiguration2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsPenHapticFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsPenHapticFeedbackEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkInputConfiguration2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1791108910, data2: 33204, data3: 23748, data4: [163, 109, 208, 87, 195, 135, 223, 218] };
}
#[repr(C)]
pub struct IInkInputProcessingConfiguration {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkInputProcessingMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: InkInputProcessingMode) -> ::windows_sys::core::HRESULT,
    pub RightDragAction: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkInputRightDragAction) -> ::windows_sys::core::HRESULT,
    pub SetRightDragAction: unsafe extern "system" fn(this: *mut *mut Self, value: InkInputRightDragAction) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkInputProcessingConfiguration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 662231134, data2: 13258, data3: 19206, data4: [166, 211, 172, 57, 69, 17, 109, 55] };
}
#[repr(C)]
pub struct IInkManager {
    pub base__: ::windows_sys::core::IInspectable,
    pub Mode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkManipulationMode) -> ::windows_sys::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut *mut Self, value: InkManipulationMode) -> ::windows_sys::core::HRESULT,
    pub ProcessPointerDown: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ProcessPointerUpdate: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessPointerUp: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessPointerUp: usize,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, drawingattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync2: unsafe extern "system" fn(this: *mut *mut Self, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync2: usize,
}
impl ::windows_sys::core::Interface for IInkManager {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1195668349, data2: 26395, data3: 16739, data4: [156, 149, 78, 141, 122, 3, 95, 225] };
}
#[repr(C)]
pub struct IInkModelerAttributes {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub PredictionTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PredictionTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetPredictionTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPredictionTime: usize,
    pub ScalingFactor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetScalingFactor: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkModelerAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3134398247, data2: 3289, data3: 19453, data4: [182, 243, 158, 3, 186, 141, 116, 84] };
}
#[repr(C)]
pub struct IInkModelerAttributes2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub UseVelocityBasedPressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetUseVelocityBasedPressure: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkModelerAttributes2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261897370, data2: 20216, data3: 24101, data4: [183, 188, 182, 84, 36, 241, 107, 179] };
}
#[repr(C)]
pub struct IInkPoint {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPoint {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2676434731, data2: 34188, data3: 18085, data4: [155, 65, 209, 149, 151, 4, 89, 253] };
}
#[repr(C)]
pub struct IInkPoint2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TiltX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPoint2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4222206967, data2: 44630, data3: 19804, data4: [189, 47, 10, 196, 95, 94, 74, 249] };
}
#[repr(C)]
pub struct IInkPointFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInkPoint: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInkPoint: usize,
}
impl ::windows_sys::core::Interface for IInkPointFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 702928156, data2: 51599, data3: 16477, data4: [159, 59, 229, 62, 49, 6, 141, 77] };
}
#[repr(C)]
pub struct IInkPointFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateInkPointWithTiltAndTimestamp: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInkPointWithTiltAndTimestamp: usize,
}
impl ::windows_sys::core::Interface for IInkPointFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3759431301, data2: 56063, data3: 17906, data4: [173, 105, 5, 13, 130, 86, 162, 9] };
}
#[repr(C)]
pub struct IInkPresenter {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInputEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub InputDeviceTypes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    InputDeviceTypes: usize,
    #[cfg(feature = "UI_Core")]
    pub SetInputDeviceTypes: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetInputDeviceTypes: usize,
    pub UnprocessedInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeInput: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InputProcessingConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeContainer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStrokeContainer: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CopyDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub UpdateDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ActivateCustomDrying: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetPredefinedConfiguration: unsafe extern "system" fn(this: *mut *mut Self, value: InkPresenterPredefinedConfiguration) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StrokesCollected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokesCollected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokesCollected: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokesCollected: usize,
    #[cfg(feature = "Foundation")]
    pub StrokesErased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokesErased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokesErased: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokesErased: usize,
}
impl ::windows_sys::core::Interface for IInkPresenter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2795204834, data2: 34939, data3: 17807, data4: [177, 115, 79, 228, 67, 137, 48, 163] };
}
#[repr(C)]
pub struct IInkPresenter2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub HighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
    pub SetHighContrastAdjustment: unsafe extern "system" fn(this: *mut *mut Self, value: InkHighContrastAdjustment) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenter2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3478382098, data2: 39476, data3: 4582, data4: [159, 51, 162, 79, 192, 217, 100, 156] };
}
#[repr(C)]
pub struct IInkPresenter3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub InputConfiguration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenter3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1373752969, data2: 54141, data3: 19088, data4: [131, 252, 127, 94, 157, 251, 242, 23] };
}
#[repr(C)]
pub struct IInkPresenterProtractor {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreTickMarksVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AreRaysVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreRaysVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCenterMarkerVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCenterMarkerVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsAngleReadoutVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsAngleReadoutVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub AccentColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetAccentColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenterProtractor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2112090794, data2: 61292, data3: 20113, data4: [167, 59, 91, 112, 213, 111, 189, 23] };
}
#[repr(C)]
pub struct IInkPresenterProtractorFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenterProtractorFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 838927305, data2: 26874, data3: 18409, data4: [129, 39, 131, 112, 113, 31, 196, 108] };
}
#[repr(C)]
pub struct IInkPresenterRuler {
    pub base__: ::windows_sys::core::IInspectable,
    pub Length: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenterRuler {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1826258266, data2: 57031, data3: 19927, data4: [135, 122, 33, 51, 241, 131, 212, 138] };
}
#[repr(C)]
pub struct IInkPresenterRuler2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub AreTickMarksVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAreTickMarksVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsCompassVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsCompassVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenterRuler2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1158876609, data2: 48225, data3: 17620, data4: [164, 35, 84, 113, 42, 230, 113, 196] };
}
#[repr(C)]
pub struct IInkPresenterRulerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkPresenterRulerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 875961323, data2: 36865, data3: 19019, data4: [166, 144, 105, 219, 175, 99, 229, 1] };
}
#[repr(C)]
pub struct IInkPresenterStencil {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkPresenterStencilKind) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Color) -> ::windows_sys::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Color) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
}
impl ::windows_sys::core::Interface for IInkPresenterStencil {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 819015021, data2: 15878, data3: 19714, data4: [177, 22, 39, 127, 181, 216, 173, 220] };
}
#[repr(C)]
pub struct IInkRecognitionResult {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextCandidates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextCandidates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
}
impl ::windows_sys::core::Interface for IInkRecognitionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 910563988, data2: 20584, data3: 16623, data4: [138, 5, 44, 47, 182, 9, 8, 162] };
}
#[repr(C)]
pub struct IInkRecognizer {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 125619875, data2: 36941, data3: 17450, data4: [177, 81, 170, 202, 54, 49, 196, 59] };
}
#[repr(C)]
pub struct IInkRecognizerContainer {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetDefaultRecognizer: unsafe extern "system" fn(this: *mut *mut Self, recognizer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut *mut Self, strokecollection: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecognizeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognizers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognizers: usize,
}
impl ::windows_sys::core::Interface for IInkRecognizerContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2806880817, data2: 32839, data3: 18072, data4: [169, 18, 248, 42, 80, 133, 1, 47] };
}
#[repr(C)]
pub struct IInkStroke {
    pub base__: ::windows_sys::core::IInspectable,
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub Selected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Recognized: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRenderingSegments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRenderingSegments: usize,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStroke {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 353652064, data2: 52451, data3: 20431, data4: [157, 82, 17, 81, 138, 182, 175, 212] };
}
#[repr(C)]
pub struct IInkStroke2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub PointTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PointTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPointTransform: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPointTransform: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInkPoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInkPoints: usize,
}
impl ::windows_sys::core::Interface for IInkStroke2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1572463860, data2: 47866, data3: 19937, data4: [137, 211, 32, 27, 30, 215, 216, 155] };
}
#[repr(C)]
pub struct IInkStroke3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StrokeStartedTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokeStartedTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStrokeStartedTime: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStrokeStartedTime: usize,
    #[cfg(feature = "Foundation")]
    pub StrokeDuration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StrokeDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetStrokeDuration: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStrokeDuration: usize,
}
impl ::windows_sys::core::Interface for IInkStroke3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1249932148, data2: 38041, data3: 16669, data4: [161, 196, 104, 133, 93, 3, 214, 95] };
}
#[repr(C)]
pub struct IInkStroke4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub PointerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStroke4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3445318373, data2: 46825, data3: 23441, data4: [165, 119, 25, 33, 210, 52, 134, 144] };
}
#[repr(C)]
pub struct IInkStrokeBuilder {
    pub base__: ::windows_sys::core::IInspectable,
    pub BeginStroke: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AppendToStroke: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub EndStroke: unsafe extern "system" fn(this: *mut *mut Self, pointerpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateStroke: unsafe extern "system" fn(this: *mut *mut Self, points: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateStroke: usize,
    pub SetDefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, drawingattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStrokeBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2193347036, data2: 7267, data3: 16860, data4: [158, 7, 75, 74, 112, 206, 216, 1] };
}
#[repr(C)]
pub struct IInkStrokeBuilder2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(this: *mut *mut Self, inkpoints: *mut ::core::ffi::c_void, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
impl ::windows_sys::core::Interface for IInkStrokeBuilder2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3179461671, data2: 29471, data3: 19644, data4: [187, 191, 109, 70, 128, 68, 241, 229] };
}
#[repr(C)]
pub struct IInkStrokeBuilder3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub CreateStrokeFromInkPoints: unsafe extern "system" fn(this: *mut *mut Self, inkpoints: *mut ::core::ffi::c_void, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: *mut ::core::ffi::c_void, strokeduration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))]
    CreateStrokeFromInkPoints: usize,
}
impl ::windows_sys::core::Interface for IInkStrokeBuilder3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2999394253, data2: 21618, data3: 18097, data4: [168, 29, 195, 122, 61, 22, 148, 65] };
}
#[repr(C)]
pub struct IInkStrokeContainer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub AddStroke: unsafe extern "system" fn(this: *mut *mut Self, stroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteSelected: usize,
    #[cfg(feature = "Foundation")]
    pub MoveSelected: unsafe extern "system" fn(this: *mut *mut Self, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveSelected: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SelectWithPolyLine: unsafe extern "system" fn(this: *mut *mut Self, polyline: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SelectWithPolyLine: usize,
    #[cfg(feature = "Foundation")]
    pub SelectWithLine: unsafe extern "system" fn(this: *mut *mut Self, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectWithLine: usize,
    pub CopySelectedToClipboard: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PasteFromClipboard: unsafe extern "system" fn(this: *mut *mut Self, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PasteFromClipboard: usize,
    pub CanPasteFromClipboard: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut *mut Self, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateRecognitionResults: unsafe extern "system" fn(this: *mut *mut Self, recognitionresults: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateRecognitionResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecognitionResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecognitionResults: usize,
}
impl ::windows_sys::core::Interface for IInkStrokeContainer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 581749702, data2: 64169, data3: 20244, data4: [182, 140, 246, 206, 230, 112, 174, 22] };
}
#[repr(C)]
pub struct IInkStrokeContainer2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AddStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddStrokes: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStrokeContainer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2298598244, data2: 55862, data3: 19407, data4: [158, 92, 209, 149, 130, 89, 149, 180] };
}
#[repr(C)]
pub struct IInkStrokeContainer3 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveWithFormatAsync: unsafe extern "system" fn(this: *mut *mut Self, outputstream: *mut ::core::ffi::c_void, inkpersistenceformat: InkPersistenceFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveWithFormatAsync: usize,
    pub GetStrokeById: unsafe extern "system" fn(this: *mut *mut Self, id: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStrokeContainer3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1023917733, data2: 47850, data3: 19586, data4: [167, 25, 123, 131, 218, 16, 103, 210] };
}
#[repr(C)]
pub struct IInkStrokeInput {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeStarted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeStarted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeContinued: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeContinued: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeContinued: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeContinued: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeEnded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeEnded: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeEnded: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub StrokeCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    StrokeCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStrokeCanceled: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStrokeCanceled: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStrokeInput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3476029051, data2: 24080, data3: 17350, data4: [160, 128, 136, 242, 110, 29, 198, 125] };
}
#[repr(C)]
pub struct IInkStrokeRenderingSegment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub BezierControlPoint1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BezierControlPoint1: usize,
    #[cfg(feature = "Foundation")]
    pub BezierControlPoint2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BezierControlPoint2: usize,
    pub Pressure: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub TiltX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub TiltY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub Twist: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkStrokeRenderingSegment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1750142751, data2: 35043, data3: 18298, data4: [162, 250, 86, 159, 95, 31, 155, 213] };
}
#[repr(C)]
pub struct IInkStrokesCollectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
impl ::windows_sys::core::Interface for IInkStrokesCollectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3304321577, data2: 6456, data3: 18780, data4: [180, 217, 109, 228, 176, 141, 72, 17] };
}
#[repr(C)]
pub struct IInkStrokesErasedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strokes: usize,
}
impl ::windows_sys::core::Interface for IInkStrokesErasedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2753653282, data2: 5379, data3: 20159, data4: [143, 245, 45, 232, 69, 132, 168, 170] };
}
#[repr(C)]
pub struct IInkSynchronizer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub BeginDry: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BeginDry: usize,
    pub EndDry: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkSynchronizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2610864480, data2: 44699, data3: 17913, data4: [132, 7, 75, 73, 59, 22, 54, 97] };
}
#[repr(C)]
pub struct IInkUnprocessedInput {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerEntered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerHovered: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerHovered: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerHovered: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerHovered: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerExited: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExited: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerPressed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerMoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerReleased: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerLost: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerLost: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkUnprocessedInput {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3678684640, data2: 33688, data3: 18721, data4: [172, 59, 171, 151, 140, 91, 162, 86] };
}
#[repr(C)]
pub struct IPenAndInkSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsHandwritingDirectlyIntoTextFieldEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub PenHandedness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PenHandedness) -> ::windows_sys::core::HRESULT,
    pub HandwritingLineHeight: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut HandwritingLineHeight) -> ::windows_sys::core::HRESULT,
    pub FontFamilyName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserConsentsToHandwritingTelemetryCollection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsTouchHandwritingEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenAndInkSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3157060495, data2: 102, data3: 17576, data4: [187, 122, 184, 57, 179, 222, 184, 245] };
}
#[repr(C)]
pub struct IPenAndInkSettings2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetPenHandedness: unsafe extern "system" fn(this: *mut *mut Self, value: PenHandedness) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenAndInkSettings2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 845339219, data2: 8004, data3: 21986, data4: [153, 41, 235, 247, 126, 84, 129, 184] };
}
#[repr(C)]
pub struct IPenAndInkSettingsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPenAndInkSettingsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3983396918, data2: 22280, data3: 23612, data4: [150, 219, 242, 245, 82, 234, 182, 65] };
}
pub type InkDrawingAttributes = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
}
impl ::core::marker::Copy for InkDrawingAttributesKind {}
impl ::core::clone::Clone for InkDrawingAttributesKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkDrawingAttributesPencilProperties = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: Self = Self(0i32);
    pub const UseSystemColors: Self = Self(1i32);
    pub const UseOriginalColors: Self = Self(2i32);
}
impl ::core::marker::Copy for InkHighContrastAdjustment {}
impl ::core::clone::Clone for InkHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkInputConfiguration = *mut ::core::ffi::c_void;
pub type InkInputProcessingConfiguration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: Self = Self(0i32);
    pub const Inking: Self = Self(1i32);
    pub const Erasing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkInputProcessingMode {}
impl ::core::clone::Clone for InkInputProcessingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: Self = Self(0i32);
    pub const AllowProcessing: Self = Self(1i32);
}
impl ::core::marker::Copy for InkInputRightDragAction {}
impl ::core::clone::Clone for InkInputRightDragAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkManager = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: Self = Self(0i32);
    pub const Erasing: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
}
impl ::core::marker::Copy for InkManipulationMode {}
impl ::core::clone::Clone for InkManipulationMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkModelerAttributes = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: Self = Self(0i32);
    pub const Isf: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkPoint = *mut ::core::ffi::c_void;
pub type InkPresenter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: Self = Self(0i32);
    pub const SimpleMultiplePointer: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPresenterPredefinedConfiguration {}
impl ::core::clone::Clone for InkPresenterPredefinedConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkPresenterProtractor = *mut ::core::ffi::c_void;
pub type InkPresenterRuler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: Self = Self(0i32);
    pub const Ruler: Self = Self(1i32);
    pub const Protractor: Self = Self(2i32);
}
impl ::core::marker::Copy for InkPresenterStencilKind {}
impl ::core::clone::Clone for InkPresenterStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkRecognitionResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for InkRecognitionTarget {}
impl ::core::clone::Clone for InkRecognitionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkRecognizer = *mut ::core::ffi::c_void;
pub type InkRecognizerContainer = *mut ::core::ffi::c_void;
pub type InkStroke = *mut ::core::ffi::c_void;
pub type InkStrokeBuilder = *mut ::core::ffi::c_void;
pub type InkStrokeContainer = *mut ::core::ffi::c_void;
pub type InkStrokeInput = *mut ::core::ffi::c_void;
pub type InkStrokeRenderingSegment = *mut ::core::ffi::c_void;
pub type InkStrokesCollectedEventArgs = *mut ::core::ffi::c_void;
pub type InkStrokesErasedEventArgs = *mut ::core::ffi::c_void;
pub type InkSynchronizer = *mut ::core::ffi::c_void;
pub type InkUnprocessedInput = *mut ::core::ffi::c_void;
pub type PenAndInkSettings = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for PenHandedness {}
impl ::core::clone::Clone for PenHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Input_Inking\"`*"]
#[repr(transparent)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: Self = Self(0i32);
    pub const Rectangle: Self = Self(1i32);
}
impl ::core::marker::Copy for PenTipShape {}
impl ::core::clone::Clone for PenTipShape {
    fn clone(&self) -> Self {
        *self
    }
}
