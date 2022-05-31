#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkBullet(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee049368_6110_4136_95f9_ee809fc20030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkDrawing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a85ed1f_1fe4_4e15_898c_8e112377e021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DrawingKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisDrawingKind) -> ::windows_core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Points: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkWord(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bd228ad_83af_4034_8f3b_f8687dfff436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TextAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TextAlternates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisLine {
    type Vtable = IInkAnalysisLine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa06d048d_2b8d_4754_ad5a_d0871193a956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisLine_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IndentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisListItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4e3c23f_c4c3_4c3a_a1a6_9d85547ee586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisListItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInkAnalysisNode(::windows_core::IUnknown);
impl IInkAnalysisNode {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::convert::From<IInkAnalysisNode> for ::windows_core::IUnknown {
    fn from(value: IInkAnalysisNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkAnalysisNode> for ::windows_core::IUnknown {
    fn from(value: &IInkAnalysisNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkAnalysisNode> for ::windows_core::IInspectable {
    fn from(value: IInkAnalysisNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkAnalysisNode> for ::windows_core::IInspectable {
    fn from(value: &IInkAnalysisNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IInkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IInkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkAnalysisNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkAnalysisNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkAnalysisNode {}
impl ::core::fmt::Debug for IInkAnalysisNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkAnalysisNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IInkAnalysisNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{30831f05-5f64-4a2c-ba37-4f4887879574}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IInkAnalysisNode {
    type Vtable = IInkAnalysisNode_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30831f05_5f64_4a2c_ba37_4f4887879574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisNode_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows_core::HRESULT,
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RotatedBoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RotatedBoundingRect: usize,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub GetStrokeIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetStrokeIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisParagraph(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9ad045c_0cd1_4dd4_a68b_eb1f12b3d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisResult {
    type Vtable = IInkAnalysisResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8948ba79_a243_4aa3_a294_1f98bd0ff580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisRoot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fb6a3c4_2fde_4061_8502_a90f32545b84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisRoot_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodekind: InkAnalysisNodeKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindNodes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisWritingRegion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd6d6231_bd16_4663_b5ae_941d3043ef5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalyzer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkAnalyzer {
    type Vtable = IInkAnalyzer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf12b8f95_0866_4dc5_8c77_f88614dfe38c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzer_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AnalysisRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsAnalyzing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AddDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AddDataForStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AddDataForStrokes: usize,
    pub ClearDataForAllStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RemoveDataForStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeids: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RemoveDataForStrokes: usize,
    pub ReplaceDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStrokeDataKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows_core::HRESULT,
    pub AnalyzeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IInkAnalyzerFactory(::windows_core::IUnknown);
impl IInkAnalyzerFactory {
    pub fn CreateAnalyzer(&self) -> ::windows_core::Result<InkAnalyzer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAnalyzer)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalyzer>(result__)
        }
    }
}
impl ::core::convert::From<IInkAnalyzerFactory> for ::windows_core::IUnknown {
    fn from(value: IInkAnalyzerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkAnalyzerFactory> for ::windows_core::IUnknown {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInkAnalyzerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkAnalyzerFactory> for ::windows_core::IInspectable {
    fn from(value: IInkAnalyzerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkAnalyzerFactory> for ::windows_core::IInspectable {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IInkAnalyzerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkAnalyzerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkAnalyzerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkAnalyzerFactory {}
impl ::core::fmt::Debug for IInkAnalyzerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkAnalyzerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IInkAnalyzerFactory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{29138686-1963-49d8-9589-e14384c769e3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IInkAnalyzerFactory {
    type Vtable = IInkAnalyzerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29138686_1963_49d8_9589_e14384c769e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAnalyzer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for InkAnalysisDrawingKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkAnalysisDrawingKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkAnalysisDrawingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisDrawingKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisDrawingKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InkAnalysisInkBullet(::windows_core::IUnknown);
impl InkAnalysisInkBullet {
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisInkBullet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkBullet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkBullet {}
impl ::core::fmt::Debug for InkAnalysisInkBullet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkBullet").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisInkBullet {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet;{ee049368-6110-4136-95f9-ee809fc20030})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisInkBullet as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
}
impl ::core::convert::From<InkAnalysisInkBullet> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisInkBullet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkBullet> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisInkBullet> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisInkBullet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkBullet> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisInkBullet) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisInkBullet) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkBullet {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkBullet {}
unsafe impl ::core::marker::Sync for InkAnalysisInkBullet {}
#[repr(transparent)]
pub struct InkAnalysisInkDrawing(::windows_core::IUnknown);
impl InkAnalysisInkDrawing {
    pub fn DrawingKind(&self) -> ::windows_core::Result<InkAnalysisDrawingKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisDrawingKind>::zeroed();
            (::windows_core::Interface::vtable(this).DrawingKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisDrawingKind>(result__)
        }
    }
    pub fn Center(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Points(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Points)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisInkDrawing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkDrawing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkDrawing {}
impl ::core::fmt::Debug for InkAnalysisInkDrawing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkDrawing").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisInkDrawing {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing;{6a85ed1f-1fe4-4e15-898c-8e112377e021})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisInkDrawing as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
}
impl ::core::convert::From<InkAnalysisInkDrawing> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkDrawing> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisInkDrawing> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkDrawing> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisInkDrawing) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisInkDrawing) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkDrawing {}
unsafe impl ::core::marker::Sync for InkAnalysisInkDrawing {}
#[repr(transparent)]
pub struct InkAnalysisInkWord(::windows_core::IUnknown);
impl InkAnalysisInkWord {
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TextAlternates(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TextAlternates)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisInkWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkWord {}
impl ::core::fmt::Debug for InkAnalysisInkWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkWord").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisInkWord {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord;{4bd228ad-83af-4034-8f3b-f8687dfff436})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisInkWord as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
}
impl ::core::convert::From<InkAnalysisInkWord> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisInkWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkWord> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisInkWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisInkWord> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisInkWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisInkWord> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisInkWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisInkWord) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisInkWord) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkWord {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkWord {}
unsafe impl ::core::marker::Sync for InkAnalysisInkWord {}
#[repr(transparent)]
pub struct InkAnalysisLine(::windows_core::IUnknown);
impl InkAnalysisLine {
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IndentLevel(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).IndentLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisLine {}
impl ::core::fmt::Debug for InkAnalysisLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisLine").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisLine {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisLine;{a06d048d-2b8d-4754-ad5a-d0871193a956})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisLine {
    type Vtable = IInkAnalysisLine_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisLine as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
}
impl ::core::convert::From<InkAnalysisLine> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisLine> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisLine> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisLine> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisLine) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisLine) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisLine {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisLine {}
unsafe impl ::core::marker::Sync for InkAnalysisLine {}
#[repr(transparent)]
pub struct InkAnalysisListItem(::windows_core::IUnknown);
impl InkAnalysisListItem {
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisListItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisListItem {}
impl ::core::fmt::Debug for InkAnalysisListItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisListItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisListItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisListItem;{b4e3c23f-c4c3-4c3a-a1a6-9d85547ee586})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisListItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
}
impl ::core::convert::From<InkAnalysisListItem> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisListItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisListItem> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisListItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisListItem> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisListItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisListItem> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisListItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisListItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisListItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisListItem {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisListItem {}
unsafe impl ::core::marker::Sync for InkAnalysisListItem {}
#[repr(transparent)]
pub struct InkAnalysisNode(::windows_core::IUnknown);
impl InkAnalysisNode {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisNode {}
impl ::core::fmt::Debug for InkAnalysisNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisNode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisNode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisNode;{30831f05-5f64-4a2c-ba37-4f4887879574})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisNode {
    type Vtable = IInkAnalysisNode_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisNode as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
}
impl ::core::convert::From<InkAnalysisNode> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisNode> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisNode> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisNode> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisNode> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisNode) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisNode> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisNode) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisNode {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisNode {}
unsafe impl ::core::marker::Sync for InkAnalysisNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for InkAnalysisNodeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkAnalysisNodeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkAnalysisNodeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisNodeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisNodeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InkAnalysisParagraph(::windows_core::IUnknown);
impl InkAnalysisParagraph {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisParagraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisParagraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisParagraph {}
impl ::core::fmt::Debug for InkAnalysisParagraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisParagraph").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisParagraph {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph;{d9ad045c-0cd1-4dd4-a68b-eb1f12b3d727})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisParagraph as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
}
impl ::core::convert::From<InkAnalysisParagraph> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisParagraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisParagraph> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisParagraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisParagraph> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisParagraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisParagraph> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisParagraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisParagraph) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisParagraph) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisParagraph {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisParagraph {}
unsafe impl ::core::marker::Sync for InkAnalysisParagraph {}
#[repr(transparent)]
pub struct InkAnalysisResult(::windows_core::IUnknown);
impl InkAnalysisResult {
    pub fn Status(&self) -> ::windows_core::Result<InkAnalysisStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisResult {}
impl ::core::fmt::Debug for InkAnalysisResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisResult;{8948ba79-a243-4aa3-a294-1f98bd0ff580})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisResult {
    type Vtable = IInkAnalysisResult_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
}
impl ::core::convert::From<InkAnalysisResult> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisResult> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisResult> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisResult> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkAnalysisResult {}
unsafe impl ::core::marker::Sync for InkAnalysisResult {}
#[repr(transparent)]
pub struct InkAnalysisRoot(::windows_core::IUnknown);
impl InkAnalysisRoot {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindNodes)(::windows_core::Interface::as_raw(this), nodekind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisRoot {}
impl ::core::fmt::Debug for InkAnalysisRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisRoot").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisRoot {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisRoot;{3fb6a3c4-2fde-4061-8502-a90f32545b84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisRoot as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
}
impl ::core::convert::From<InkAnalysisRoot> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisRoot> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisRoot> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisRoot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisRoot> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisRoot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisRoot) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisRoot) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisRoot {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisRoot {}
unsafe impl ::core::marker::Sync for InkAnalysisRoot {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for InkAnalysisStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkAnalysisStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkAnalysisStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for InkAnalysisStrokeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InkAnalysisStrokeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkAnalysisStrokeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisStrokeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisStrokeKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct InkAnalysisWritingRegion(::windows_core::IUnknown);
impl InkAnalysisWritingRegion {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<InkAnalysisNodeKind> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<InkAnalysisNodeKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    pub fn BoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RotatedBoundingRect(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RotatedBoundingRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<IInkAnalysisNode> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetStrokeIds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<u32>> {
        let this = &::windows_core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetStrokeIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RecognizedText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalysisWritingRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalysisWritingRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisWritingRegion {}
impl ::core::fmt::Debug for InkAnalysisWritingRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisWritingRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalysisWritingRegion {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion;{dd6d6231-bd16-4663-b5ae-941d3043ef5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalysisWritingRegion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
}
impl ::core::convert::From<InkAnalysisWritingRegion> for ::windows_core::IUnknown {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisWritingRegion> for ::windows_core::IUnknown {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalysisWritingRegion> for ::windows_core::IInspectable {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisWritingRegion> for ::windows_core::IInspectable {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: InkAnalysisWritingRegion) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows_core::Error;
    fn try_from(value: &InkAnalysisWritingRegion) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows_core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisWritingRegion {}
unsafe impl ::core::marker::Sync for InkAnalysisWritingRegion {}
#[repr(transparent)]
pub struct InkAnalyzer(::windows_core::IUnknown);
impl InkAnalyzer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InkAnalyzer, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AnalysisRoot(&self) -> ::windows_core::Result<InkAnalysisRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnalysisRoot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkAnalysisRoot>(result__)
        }
    }
    pub fn IsAnalyzing(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAnalyzing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AddDataForStroke<'a, Param0: ::windows_core::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDataForStroke)(::windows_core::Interface::as_raw(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AddDataForStrokes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::InkStroke>>>(&self, strokes: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddDataForStrokes)(::windows_core::Interface::as_raw(this), strokes.into_param().abi()).ok() }
    }
    pub fn ClearDataForAllStrokes(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearDataForAllStrokes)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDataForStroke)(::windows_core::Interface::as_raw(this), strokeid).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveDataForStrokes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<u32>>>(&self, strokeids: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDataForStrokes)(::windows_core::Interface::as_raw(this), strokeids.into_param().abi()).ok() }
    }
    pub fn ReplaceDataForStroke<'a, Param0: ::windows_core::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceDataForStroke)(::windows_core::Interface::as_raw(this), stroke.into_param().abi()).ok() }
    }
    pub fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStrokeDataKind)(::windows_core::Interface::as_raw(this), strokeid, strokekind).ok() }
    }
    pub fn AnalyzeAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<InkAnalysisResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnalyzeAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<InkAnalysisResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkAnalyzer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkAnalyzer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalyzer {}
impl ::core::fmt::Debug for InkAnalyzer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalyzer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkAnalyzer {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalyzer;{f12b8f95-0866-4dc5-8c77-f88614dfe38c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkAnalyzer {
    type Vtable = IInkAnalyzer_Vtbl;
    const IID: ::windows_core::GUID = <IInkAnalyzer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalyzer";
}
impl ::core::convert::From<InkAnalyzer> for ::windows_core::IUnknown {
    fn from(value: InkAnalyzer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalyzer> for ::windows_core::IUnknown {
    fn from(value: &InkAnalyzer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkAnalyzer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkAnalyzer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkAnalyzer> for ::windows_core::IInspectable {
    fn from(value: InkAnalyzer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalyzer> for ::windows_core::IInspectable {
    fn from(value: &InkAnalyzer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkAnalyzer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkAnalyzer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkAnalyzer {}
unsafe impl ::core::marker::Sync for InkAnalyzer {}
