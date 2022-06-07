#[repr(C)]
pub struct IInkAnalysisInkBullet {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisInkBullet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3993277288, data2: 24848, data3: 16694, data4: [149, 249, 238, 128, 159, 194, 0, 48] };
}
#[repr(C)]
pub struct IInkAnalysisInkDrawing {
    pub base__: ::windows_sys::core::IInspectable,
    pub DrawingKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkAnalysisDrawingKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Center: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Point) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Center: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
}
impl ::windows_sys::core::Interface for IInkAnalysisInkDrawing {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1787161887, data2: 8164, data3: 19989, data4: [137, 140, 142, 17, 35, 119, 224, 33] };
}
#[repr(C)]
pub struct IInkAnalysisInkWord {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAlternates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAlternates: usize,
}
impl ::windows_sys::core::Interface for IInkAnalysisInkWord {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1272064173, data2: 33711, data3: 16436, data4: [143, 59, 248, 104, 125, 255, 244, 54] };
}
#[repr(C)]
pub struct IInkAnalysisLine {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IndentLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisLine {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2691499149, data2: 11149, data3: 18260, data4: [173, 90, 208, 135, 17, 147, 169, 86] };
}
#[repr(C)]
pub struct IInkAnalysisListItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisListItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3034825279, data2: 50371, data3: 19514, data4: [161, 166, 157, 133, 84, 126, 229, 134] };
}
#[repr(C)]
pub struct IInkAnalysisNode {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkAnalysisNodeKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RotatedBoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RotatedBoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub Parent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokeIds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokeIds: usize,
}
impl ::windows_sys::core::Interface for IInkAnalysisNode {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 813899525, data2: 24420, data3: 18988, data4: [186, 55, 79, 72, 135, 135, 149, 116] };
}
#[repr(C)]
pub struct IInkAnalysisParagraph {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisParagraph {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3651994716, data2: 3281, data3: 19924, data4: [166, 139, 235, 31, 18, 179, 215, 39] };
}
#[repr(C)]
pub struct IInkAnalysisResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut InkAnalysisStatus) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2303244921, data2: 41539, data3: 19107, data4: [162, 148, 31, 152, 189, 15, 245, 128] };
}
#[repr(C)]
pub struct IInkAnalysisRoot {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindNodes: unsafe extern "system" fn(this: *mut *mut Self, nodekind: InkAnalysisNodeKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindNodes: usize,
}
impl ::windows_sys::core::Interface for IInkAnalysisRoot {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1068934084, data2: 12254, data3: 16481, data4: [133, 2, 169, 15, 50, 84, 91, 132] };
}
#[repr(C)]
pub struct IInkAnalysisWritingRegion {
    pub base__: ::windows_sys::core::IInspectable,
    pub RecognizedText: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalysisWritingRegion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3714933297, data2: 48406, data3: 18019, data4: [181, 174, 148, 29, 48, 67, 239, 91] };
}
#[repr(C)]
pub struct IInkAnalyzer {
    pub base__: ::windows_sys::core::IInspectable,
    pub AnalysisRoot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsAnalyzing: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub AddDataForStroke: unsafe extern "system" fn(this: *mut *mut Self, stroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddDataForStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddDataForStrokes: usize,
    pub ClearDataForAllStrokes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub RemoveDataForStroke: unsafe extern "system" fn(this: *mut *mut Self, strokeid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveDataForStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokeids: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveDataForStrokes: usize,
    pub ReplaceDataForStroke: unsafe extern "system" fn(this: *mut *mut Self, stroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDataKind: unsafe extern "system" fn(this: *mut *mut Self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnalyzeAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnalyzeAsync: usize,
}
impl ::windows_sys::core::Interface for IInkAnalyzer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4046163861, data2: 2150, data3: 19909, data4: [140, 119, 248, 134, 20, 223, 227, 140] };
}
#[repr(C)]
pub struct IInkAnalyzerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateAnalyzer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkAnalyzerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 689145478, data2: 6499, data3: 18904, data4: [149, 137, 225, 67, 132, 199, 105, 227] };
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: Self = Self(0i32);
    pub const Circle: Self = Self(1i32);
    pub const Ellipse: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
    pub const IsoscelesTriangle: Self = Self(4i32);
    pub const EquilateralTriangle: Self = Self(5i32);
    pub const RightTriangle: Self = Self(6i32);
    pub const Quadrilateral: Self = Self(7i32);
    pub const Rectangle: Self = Self(8i32);
    pub const Square: Self = Self(9i32);
    pub const Diamond: Self = Self(10i32);
    pub const Trapezoid: Self = Self(11i32);
    pub const Parallelogram: Self = Self(12i32);
    pub const Pentagon: Self = Self(13i32);
    pub const Hexagon: Self = Self(14i32);
}
impl ::core::marker::Copy for InkAnalysisDrawingKind {}
impl ::core::clone::Clone for InkAnalysisDrawingKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkAnalysisInkBullet = *mut ::core::ffi::c_void;
pub type InkAnalysisInkDrawing = *mut ::core::ffi::c_void;
pub type InkAnalysisInkWord = *mut ::core::ffi::c_void;
pub type InkAnalysisLine = *mut ::core::ffi::c_void;
pub type InkAnalysisListItem = *mut ::core::ffi::c_void;
pub type InkAnalysisNode = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const WritingRegion: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const InkWord: Self = Self(5i32);
    pub const InkBullet: Self = Self(6i32);
    pub const InkDrawing: Self = Self(7i32);
    pub const ListItem: Self = Self(8i32);
}
impl ::core::marker::Copy for InkAnalysisNodeKind {}
impl ::core::clone::Clone for InkAnalysisNodeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkAnalysisParagraph = *mut ::core::ffi::c_void;
pub type InkAnalysisResult = *mut ::core::ffi::c_void;
pub type InkAnalysisRoot = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: Self = Self(0i32);
    pub const Unchanged: Self = Self(1i32);
}
impl ::core::marker::Copy for InkAnalysisStatus {}
impl ::core::clone::Clone for InkAnalysisStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: Self = Self(0i32);
    pub const Writing: Self = Self(1i32);
    pub const Drawing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkAnalysisStrokeKind {}
impl ::core::clone::Clone for InkAnalysisStrokeKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type InkAnalysisWritingRegion = *mut ::core::ffi::c_void;
pub type InkAnalyzer = *mut ::core::ffi::c_void;
