#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn CreateRecognizer(pclsid: *mut ::windows_sys::core::GUID, phrec: *mut HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows_sys::core::GUID, count: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn LoadCachedAttributes(clsid: ::windows_sys::core::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn MakeWordList(hrec: HRECOGNIZER, pbuffer: ::windows_sys::core::PCWSTR, phwl: *mut HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: ::windows_sys::core::PCWSTR, cwcafter: u32, pwcafter: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
    pub fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type ALT_BREAKS = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ALT_BREAKS_SAME: ALT_BREAKS = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ALT_BREAKS_UNIQUE: ALT_BREAKS = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ALT_BREAKS_FULL: ALT_BREAKS = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_ADDSTROKE_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_INTERRUPTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_PROCESS_FAILED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_RESETCONTEXT_FAILED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETCACMODE_FAILED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETFACTOID_FAILED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETFLAGS_FAILED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETGUIDE_FAILED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETTEXTCONTEXT_FAILED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ASYNC_RECO_SETWORDLIST_FAILED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type AppearanceConstants = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfFlat: AppearanceConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfThreeD: AppearanceConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const BEST_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type BorderStyleConstants = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfNoBorder: BorderStyleConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfFixedSingle: BorderStyleConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CAC_FULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CAC_PREFIX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CAC_RANDOM: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct CHARACTER_RANGE {
    pub wcLow: u16,
    pub cChars: u16,
}
impl ::core::marker::Copy for CHARACTER_RANGE {}
impl ::core::clone::Clone for CHARACTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type CONFIDENCE_LEVEL = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CFL_STRONG: CONFIDENCE_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CFL_INTERMEDIATE: CONFIDENCE_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CFL_POOR: CONFIDENCE_LEVEL = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type CorrectionMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionMode_NotVisible: CorrectionMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionMode_PreInsertion: CorrectionMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type CorrectionPosition = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionPosition_Auto: CorrectionPosition = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionPosition_Bottom: CorrectionPosition = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const CorrectionPosition_Top: CorrectionPosition = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_Ink = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IStrokes: DISPID_Ink = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IExtendedProperties: DISPID_Ink = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IGetBoundingBox: DISPID_Ink = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IDeleteStrokes: DISPID_Ink = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IDeleteStroke: DISPID_Ink = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IExtractStrokes: DISPID_Ink = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IExtractWithRectangle: DISPID_Ink = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IDirty: DISPID_Ink = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICustomStrokes: DISPID_Ink = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IClone: DISPID_Ink = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IHitTestCircle: DISPID_Ink = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IHitTestWithRectangle: DISPID_Ink = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IHitTestWithLasso: DISPID_Ink = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_INearestPoint: DISPID_Ink = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICreateStrokes: DISPID_Ink = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICreateStroke: DISPID_Ink = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IAddStrokesAtRectangle: DISPID_Ink = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IClip: DISPID_Ink = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISave: DISPID_Ink = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ILoad: DISPID_Ink = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICreateStrokeFromPoints: DISPID_Ink = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IClipboardCopyWithRectangle: DISPID_Ink = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IClipboardCopy: DISPID_Ink = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICanPaste: DISPID_Ink = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IClipboardPaste: DISPID_Ink = 25i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCollector = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICEnabled: DISPID_InkCollector = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICHwnd: DISPID_InkCollector = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICPaint: DISPID_InkCollector = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICText: DISPID_InkCollector = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICDefaultDrawingAttributes: DISPID_InkCollector = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICRenderer: DISPID_InkCollector = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICInk: DISPID_InkCollector = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICAutoRedraw: DISPID_InkCollector = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICCollectingInk: DISPID_InkCollector = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSetEventInterest: DISPID_InkCollector = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICGetEventInterest: DISPID_InkCollector = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEditingMode: DISPID_InkCollector = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOSelection: DISPID_InkCollector = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOAttachMode: DISPID_InkCollector = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOHitTestSelection: DISPID_InkCollector = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IODraw: DISPID_InkCollector = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPPicture: DISPID_InkCollector = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPSizeMode: DISPID_InkCollector = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPBackColor: DISPID_InkCollector = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICCursors: DISPID_InkCollector = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICMarginX: DISPID_InkCollector = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICMarginY: DISPID_InkCollector = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSetWindowInputRectangle: DISPID_InkCollector = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICGetWindowInputRectangle: DISPID_InkCollector = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICTablet: DISPID_InkCollector = 25i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSetAllTabletsMode: DISPID_InkCollector = 26i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSetSingleTabletIntegratedMode: DISPID_InkCollector = 27i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICCollectionMode: DISPID_InkCollector = 28i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSetGestureStatus: DISPID_InkCollector = 29i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICGetGestureStatus: DISPID_InkCollector = 30i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICDynamicRendering: DISPID_InkCollector = 31i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICDesiredPacketDescription: DISPID_InkCollector = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEraserMode: DISPID_InkCollector = 33i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEraserWidth: DISPID_InkCollector = 34i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICMouseIcon: DISPID_InkCollector = 35i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICMousePointer: DISPID_InkCollector = 36i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPInkEnabled: DISPID_InkCollector = 37i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSupportHighContrastInk: DISPID_InkCollector = 38i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOSupportHighContrastSelectionUI: DISPID_InkCollector = 39i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCollectorEvent = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICEStroke: DISPID_InkCollectorEvent = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICECursorDown: DISPID_InkCollectorEvent = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICENewPackets: DISPID_InkCollectorEvent = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICENewInAirPackets: DISPID_InkCollectorEvent = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICECursorButtonDown: DISPID_InkCollectorEvent = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICECursorButtonUp: DISPID_InkCollectorEvent = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICECursorInRange: DISPID_InkCollectorEvent = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICECursorOutOfRange: DISPID_InkCollectorEvent = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICESystemGesture: DISPID_InkCollectorEvent = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICEGesture: DISPID_InkCollectorEvent = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICETabletAdded: DISPID_InkCollectorEvent = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICETabletRemoved: DISPID_InkCollectorEvent = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEPainting: DISPID_InkCollectorEvent = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEPainted: DISPID_InkCollectorEvent = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionChanging: DISPID_InkCollectorEvent = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionChanged: DISPID_InkCollectorEvent = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionMoving: DISPID_InkCollectorEvent = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionMoved: DISPID_InkCollectorEvent = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionResizing: DISPID_InkCollectorEvent = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOESelectionResized: DISPID_InkCollectorEvent = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEStrokesDeleting: DISPID_InkCollectorEvent = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IOEStrokesDeleted: DISPID_InkCollectorEvent = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEChangeUICues: DISPID_InkCollectorEvent = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEClick: DISPID_InkCollectorEvent = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEDblClick: DISPID_InkCollectorEvent = 25i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEInvalidated: DISPID_InkCollectorEvent = 26i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseDown: DISPID_InkCollectorEvent = 27i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseEnter: DISPID_InkCollectorEvent = 28i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseHover: DISPID_InkCollectorEvent = 29i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseLeave: DISPID_InkCollectorEvent = 30i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseMove: DISPID_InkCollectorEvent = 31i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseUp: DISPID_InkCollectorEvent = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEMouseWheel: DISPID_InkCollectorEvent = 33i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPESizeModeChanged: DISPID_InkCollectorEvent = 34i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEStyleChanged: DISPID_InkCollectorEvent = 35i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPESystemColorsChanged: DISPID_InkCollectorEvent = 36i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEKeyDown: DISPID_InkCollectorEvent = 37i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEKeyPress: DISPID_InkCollectorEvent = 38i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEKeyUp: DISPID_InkCollectorEvent = 39i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPEResize: DISPID_InkCollectorEvent = 40i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IPESizeChanged: DISPID_InkCollectorEvent = 41i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCursor = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrName: DISPID_InkCursor = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrId: DISPID_InkCursor = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrDrawingAttributes: DISPID_InkCursor = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrButtons: DISPID_InkCursor = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrInverted: DISPID_InkCursor = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsrTablet: DISPID_InkCursor = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCursorButton = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBName: DISPID_InkCursorButton = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBId: DISPID_InkCursorButton = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBState: DISPID_InkCursorButton = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCursorButtons = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBs_NewEnum: DISPID_InkCursorButtons = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBsItem: DISPID_InkCursorButtons = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICBsCount: DISPID_InkCursorButtons = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCursors = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICs_NewEnum: DISPID_InkCursors = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsItem: DISPID_InkCursors = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICsCount: DISPID_InkCursors = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkCustomStrokes = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSs_NewEnum: DISPID_InkCustomStrokes = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSsItem: DISPID_InkCustomStrokes = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSsCount: DISPID_InkCustomStrokes = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSsAdd: DISPID_InkCustomStrokes = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSsRemove: DISPID_InkCustomStrokes = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ICSsClear: DISPID_InkCustomStrokes = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkDivider = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivider_Strokes: DISPID_InkDivider = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivider_RecognizerContext: DISPID_InkDivider = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivider_LineHeight: DISPID_InkDivider = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivider_Divide: DISPID_InkDivider = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkDivisionResult = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionResult_Strokes: DISPID_InkDivisionResult = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionResult_ResultByType: DISPID_InkDivisionResult = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkDivisionUnit = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnit_Strokes: DISPID_InkDivisionUnit = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnit_DivisionType: DISPID_InkDivisionUnit = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnit_RecognizedString: DISPID_InkDivisionUnit = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnit_RotationTransform: DISPID_InkDivisionUnit = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkDivisionUnits = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnits_NewEnum: DISPID_InkDivisionUnits = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnits_Item: DISPID_InkDivisionUnits = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IInkDivisionUnits_Count: DISPID_InkDivisionUnits = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkDrawingAttributes = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAHeight: DISPID_InkDrawingAttributes = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAColor: DISPID_InkDrawingAttributes = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAWidth: DISPID_InkDrawingAttributes = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAFitToCurve: DISPID_InkDrawingAttributes = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAIgnorePressure: DISPID_InkDrawingAttributes = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAAntiAliased: DISPID_InkDrawingAttributes = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DATransparency: DISPID_InkDrawingAttributes = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DARasterOperation: DISPID_InkDrawingAttributes = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAPenTip: DISPID_InkDrawingAttributes = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAClone: DISPID_InkDrawingAttributes = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DAExtendedProperties: DISPID_InkDrawingAttributes = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkEdit = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Text: DISPID_InkEdit = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_TextRTF: DISPID_InkEdit = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Hwnd: DISPID_InkEdit = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DisableNoScroll: DISPID_InkEdit = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Locked: DISPID_InkEdit = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Enabled: DISPID_InkEdit = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MaxLength: DISPID_InkEdit = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MultiLine: DISPID_InkEdit = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ScrollBars: DISPID_InkEdit = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RTSelStart: DISPID_InkEdit = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RTSelLength: DISPID_InkEdit = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RTSelText: DISPID_InkEdit = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelAlignment: DISPID_InkEdit = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelBold: DISPID_InkEdit = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelCharOffset: DISPID_InkEdit = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelColor: DISPID_InkEdit = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelFontName: DISPID_InkEdit = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelFontSize: DISPID_InkEdit = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelItalic: DISPID_InkEdit = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelRTF: DISPID_InkEdit = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelUnderline: DISPID_InkEdit = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DragIcon: DISPID_InkEdit = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Status: DISPID_InkEdit = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_UseMouseForInput: DISPID_InkEdit = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkMode: DISPID_InkEdit = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkInsertMode: DISPID_InkEdit = 25i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoTimeout: DISPID_InkEdit = 26i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_DrawAttr: DISPID_InkEdit = 27i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Recognizer: DISPID_InkEdit = 28i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Factoid: DISPID_InkEdit = 29i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelInk: DISPID_InkEdit = 30i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SelInksDisplayMode: DISPID_InkEdit = 31i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Recognize: DISPID_InkEdit = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_GetGestStatus: DISPID_InkEdit = 33i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SetGestStatus: DISPID_InkEdit = 34i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_Refresh: DISPID_InkEdit = 35i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkEditEvents = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeChange: DISPID_InkEditEvents = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeSelChange: DISPID_InkEditEvents = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeKeyDown: DISPID_InkEditEvents = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeKeyUp: DISPID_InkEditEvents = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeMouseUp: DISPID_InkEditEvents = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeMouseDown: DISPID_InkEditEvents = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeKeyPress: DISPID_InkEditEvents = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeDblClick: DISPID_InkEditEvents = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeClick: DISPID_InkEditEvents = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeMouseMove: DISPID_InkEditEvents = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeCursorDown: DISPID_InkEditEvents = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeStroke: DISPID_InkEditEvents = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeGesture: DISPID_InkEditEvents = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IeeRecognitionResult: DISPID_InkEditEvents = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkEvent = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEInkAdded: DISPID_InkEvent = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEInkDeleted: DISPID_InkEvent = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkExtendedProperties = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPs_NewEnum: DISPID_InkExtendedProperties = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsItem: DISPID_InkExtendedProperties = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsCount: DISPID_InkExtendedProperties = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsAdd: DISPID_InkExtendedProperties = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsRemove: DISPID_InkExtendedProperties = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsClear: DISPID_InkExtendedProperties = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPsDoesPropertyExist: DISPID_InkExtendedProperties = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkExtendedProperty = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPGuid: DISPID_InkExtendedProperty = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IEPData: DISPID_InkExtendedProperty = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkGesture = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IGId: DISPID_InkGesture = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IGGetHotPoint: DISPID_InkGesture = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IGConfidence: DISPID_InkGesture = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecoAlternate = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_String: DISPID_InkRecoAlternate = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_LineNumber: DISPID_InkRecoAlternate = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Baseline: DISPID_InkRecoAlternate = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Midline: DISPID_InkRecoAlternate = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Ascender: DISPID_InkRecoAlternate = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Descender: DISPID_InkRecoAlternate = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Confidence: DISPID_InkRecoAlternate = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_Strokes: DISPID_InkRecoAlternate = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges: DISPID_InkRecoAlternate = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_GetStrokesFromTextRange: DISPID_InkRecoAlternate = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_GetTextRangeFromStrokes: DISPID_InkRecoAlternate = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_GetPropertyValue: DISPID_InkRecoAlternate = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_LineAlternates: DISPID_InkRecoAlternate = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_ConfidenceAlternates: DISPID_InkRecoAlternate = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues: DISPID_InkRecoAlternate = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecoContext = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Strokes: DISPID_InkRecoContext = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_CharacterAutoCompletionMode: DISPID_InkRecoContext = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Factoid: DISPID_InkRecoContext = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_WordList: DISPID_InkRecoContext = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Recognizer: DISPID_InkRecoContext = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Guide: DISPID_InkRecoContext = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Flags: DISPID_InkRecoContext = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_PrefixText: DISPID_InkRecoContext = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_SuffixText: DISPID_InkRecoContext = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_StopRecognition: DISPID_InkRecoContext = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Clone: DISPID_InkRecoContext = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_Recognize: DISPID_InkRecoContext = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_StopBackgroundRecognition: DISPID_InkRecoContext = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_EndInkInput: DISPID_InkRecoContext = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_BackgroundRecognize: DISPID_InkRecoContext = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_BackgroundRecognizeWithAlternates: DISPID_InkRecoContext = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx_IsStringSupported: DISPID_InkRecoContext = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecoContext2 = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: DISPID_InkRecoContext2 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognitionAlternates = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionAlternates_NewEnum: DISPID_InkRecognitionAlternates = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionAlternates_Item: DISPID_InkRecognitionAlternates = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionAlternates_Count: DISPID_InkRecognitionAlternates = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionAlternates_Strokes: DISPID_InkRecognitionAlternates = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognitionEvent = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRERecognitionWithAlternates: DISPID_InkRecognitionEvent = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRERecognition: DISPID_InkRecognitionEvent = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognitionResult = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_TopString: DISPID_InkRecognitionResult = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_TopAlternate: DISPID_InkRecognitionResult = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_Strokes: DISPID_InkRecognitionResult = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_TopConfidence: DISPID_InkRecognitionResult = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: DISPID_InkRecognitionResult = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: DISPID_InkRecognitionResult = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: DISPID_InkRecognitionResult = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognizer = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoClsid: DISPID_InkRecognizer = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoName: DISPID_InkRecognizer = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoVendor: DISPID_InkRecognizer = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoCapabilities: DISPID_InkRecognizer = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoLanguageID: DISPID_InkRecognizer = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoPreferredPacketDescription: DISPID_InkRecognizer = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoCreateRecognizerContext: DISPID_InkRecognizer = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoSupportedProperties: DISPID_InkRecognizer = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognizer2 = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoId: DISPID_InkRecognizer2 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_RecoUnicodeRanges: DISPID_InkRecognizer2 = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognizerGuide = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGWritingBox: DISPID_InkRecognizerGuide = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGDrawnBox: DISPID_InkRecognizerGuide = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGRows: DISPID_InkRecognizerGuide = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGColumns: DISPID_InkRecognizerGuide = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGMidline: DISPID_InkRecognizerGuide = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGGuideData: DISPID_InkRecognizerGuide = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRecognizers = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecos_NewEnum: DISPID_InkRecognizers = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecosItem: DISPID_InkRecognizers = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecosCount: DISPID_InkRecognizers = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRecosGetDefaultRecognizer: DISPID_InkRecognizers = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRectangle = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRTop: DISPID_InkRectangle = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRLeft: DISPID_InkRectangle = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRBottom: DISPID_InkRectangle = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRRight: DISPID_InkRectangle = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGetRectangle: DISPID_InkRectangle = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRSetRectangle: DISPID_InkRectangle = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRData: DISPID_InkRectangle = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkRenderer = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGetViewTransform: DISPID_InkRenderer = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRSetViewTransform: DISPID_InkRenderer = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRGetObjectTransform: DISPID_InkRenderer = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRSetObjectTransform: DISPID_InkRenderer = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRDraw: DISPID_InkRenderer = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRDrawStroke: DISPID_InkRenderer = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRPixelToInkSpace: DISPID_InkRenderer = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRInkSpaceToPixel: DISPID_InkRenderer = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRPixelToInkSpaceFromPoints: DISPID_InkRenderer = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRInkSpaceToPixelFromPoints: DISPID_InkRenderer = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRMeasure: DISPID_InkRenderer = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRMeasureStroke: DISPID_InkRenderer = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRMove: DISPID_InkRenderer = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRRotate: DISPID_InkRenderer = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IRScale: DISPID_InkRenderer = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkStrokeDisp = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDInkIndex: DISPID_InkStrokeDisp = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDID: DISPID_InkStrokeDisp = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetBoundingBox: DISPID_InkStrokeDisp = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDDrawingAttributes: DISPID_InkStrokeDisp = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDFindIntersections: DISPID_InkStrokeDisp = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetRectangleIntersections: DISPID_InkStrokeDisp = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDClip: DISPID_InkStrokeDisp = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDHitTestCircle: DISPID_InkStrokeDisp = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDNearestPoint: DISPID_InkStrokeDisp = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDSplit: DISPID_InkStrokeDisp = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDExtendedProperties: DISPID_InkStrokeDisp = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDInk: DISPID_InkStrokeDisp = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDBezierPoints: DISPID_InkStrokeDisp = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDPolylineCusps: DISPID_InkStrokeDisp = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDBezierCusps: DISPID_InkStrokeDisp = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDSelfIntersections: DISPID_InkStrokeDisp = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDPacketCount: DISPID_InkStrokeDisp = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDPacketSize: DISPID_InkStrokeDisp = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDPacketDescription: DISPID_InkStrokeDisp = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDDeleted: DISPID_InkStrokeDisp = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetPacketDescriptionPropertyMetrics: DISPID_InkStrokeDisp = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetPoints: DISPID_InkStrokeDisp = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDSetPoints: DISPID_InkStrokeDisp = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetPacketData: DISPID_InkStrokeDisp = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetPacketValuesByProperty: DISPID_InkStrokeDisp = 25i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDSetPacketValuesByProperty: DISPID_InkStrokeDisp = 26i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDGetFlattenedBezierPoints: DISPID_InkStrokeDisp = 27i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDScaleToRectangle: DISPID_InkStrokeDisp = 28i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDTransform: DISPID_InkStrokeDisp = 29i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDMove: DISPID_InkStrokeDisp = 30i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDRotate: DISPID_InkStrokeDisp = 31i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDShear: DISPID_InkStrokeDisp = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISDScale: DISPID_InkStrokeDisp = 33i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkStrokes = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISs_NewEnum: DISPID_InkStrokes = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsItem: DISPID_InkStrokes = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsCount: DISPID_InkStrokes = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsValid: DISPID_InkStrokes = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsInk: DISPID_InkStrokes = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsAdd: DISPID_InkStrokes = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsAddStrokes: DISPID_InkStrokes = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsRemove: DISPID_InkStrokes = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsRemoveStrokes: DISPID_InkStrokes = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsToString: DISPID_InkStrokes = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsModifyDrawingAttributes: DISPID_InkStrokes = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsGetBoundingBox: DISPID_InkStrokes = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsScaleToRectangle: DISPID_InkStrokes = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsTransform: DISPID_InkStrokes = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsMove: DISPID_InkStrokes = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsRotate: DISPID_InkStrokes = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsShear: DISPID_InkStrokes = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsScale: DISPID_InkStrokes = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsClip: DISPID_InkStrokes = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsRecognitionResult: DISPID_InkStrokes = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ISsRemoveRecognitionResult: DISPID_InkStrokes = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkTablet = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITName: DISPID_InkTablet = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITPlugAndPlayId: DISPID_InkTablet = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITPropertyMetrics: DISPID_InkTablet = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITIsPacketPropertySupported: DISPID_InkTablet = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITMaximumInputRectangle: DISPID_InkTablet = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITHardwareCapabilities: DISPID_InkTablet = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkTablet2 = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IT2DeviceKind: DISPID_InkTablet2 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkTablet3 = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IT3IsMultiTouch: DISPID_InkTablet3 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_IT3MaximumCursors: DISPID_InkTablet3 = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkTablets = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITs_NewEnum: DISPID_InkTablets = -4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITsItem: DISPID_InkTablets = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITsDefaultTablet: DISPID_InkTablets = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITsCount: DISPID_InkTablets = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITsIsPacketPropertySupported: DISPID_InkTablets = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkTransform = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITReset: DISPID_InkTransform = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITTranslate: DISPID_InkTransform = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITRotate: DISPID_InkTransform = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITReflect: DISPID_InkTransform = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITShear: DISPID_InkTransform = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITScale: DISPID_InkTransform = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeM11: DISPID_InkTransform = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeM12: DISPID_InkTransform = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeM21: DISPID_InkTransform = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeM22: DISPID_InkTransform = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeDx: DISPID_InkTransform = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITeDy: DISPID_InkTransform = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITGetTransform: DISPID_InkTransform = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITSetTransform: DISPID_InkTransform = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_ITData: DISPID_InkTransform = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkWordList = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkWordList_AddWord: DISPID_InkWordList = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkWordList_RemoveWord: DISPID_InkWordList = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkWordList_Merge: DISPID_InkWordList = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_InkWordList2 = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_InkWordList2_AddWords: DISPID_InkWordList2 = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_MathInputControlEvents = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MICInsert: DISPID_MathInputControlEvents = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MICClose: DISPID_MathInputControlEvents = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MICPaint: DISPID_MathInputControlEvents = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_MICClear: DISPID_MathInputControlEvents = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_PenInputPanel = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPAttachedEditWindow: DISPID_PenInputPanel = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPFactoid: DISPID_PenInputPanel = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPCurrentPanel: DISPID_PenInputPanel = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPDefaultPanel: DISPID_PenInputPanel = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPVisible: DISPID_PenInputPanel = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPTop: DISPID_PenInputPanel = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPLeft: DISPID_PenInputPanel = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPWidth: DISPID_PenInputPanel = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPHeight: DISPID_PenInputPanel = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPMoveTo: DISPID_PenInputPanel = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPCommitPendingInput: DISPID_PenInputPanel = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPRefresh: DISPID_PenInputPanel = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPBusy: DISPID_PenInputPanel = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPVerticalOffset: DISPID_PenInputPanel = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPHorizontalOffset: DISPID_PenInputPanel = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPEnableTsf: DISPID_PenInputPanel = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPAutoShow: DISPID_PenInputPanel = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_PenInputPanelEvents = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type DISPID_StrokeEvent = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SEStrokesAdded: DISPID_StrokeEvent = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DISPID_SEStrokesRemoved: DISPID_StrokeEvent = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct DYNAMIC_RENDERER_CACHED_DATA {
    pub strokeId: i32,
    pub dynamicRenderer: *mut *mut *mut *mut IDynamicRenderer,
}
impl ::core::marker::Copy for DYNAMIC_RENDERER_CACHED_DATA {}
impl ::core::clone::Clone for DYNAMIC_RENDERER_CACHED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DynamicRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3973262058, data2: 29807, data3: 19915, data4: [191, 104, 8, 39, 87, 250, 255, 24] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETDRAWATTR: u32 = 1541u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETFACTOID: u32 = 1549u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETGESTURESTATUS: u32 = 1545u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETINKINSERTMODE: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETINKMODE: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETMOUSEICON: u32 = 1553u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETMOUSEPOINTER: u32 = 1555u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETRECOGNIZER: u32 = 1547u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETRECOTIMEOUT: u32 = 1543u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETSELINK: u32 = 1551u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETSELINKDISPLAYMODE: u32 = 1562u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETSTATUS: u32 = 1557u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_GETUSEMOUSEFORINPUT: u32 = 1559u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_RECOGNIZE: u32 = 1558u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETDRAWATTR: u32 = 1542u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETFACTOID: u32 = 1550u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETGESTURESTATUS: u32 = 1546u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETINKINSERTMODE: u32 = 1540u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETINKMODE: u32 = 1538u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETMOUSEICON: u32 = 1554u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETMOUSEPOINTER: u32 = 1556u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETRECOGNIZER: u32 = 1548u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETRECOTIMEOUT: u32 = 1544u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETSELINK: u32 = 1552u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETSELINKDISPLAYMODE: u32 = 1561u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EM_SETUSEMOUSEFORINPUT: u32 = 1560u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type EventMask = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceStateChanging: EventMask = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceStateChanged: EventMask = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceSizeChanging: EventMask = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceSizeChanged: EventMask = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InputAreaChanging: EventMask = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InputAreaChanged: EventMask = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_CorrectionModeChanging: EventMask = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_CorrectionModeChanged: EventMask = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceVisibilityChanging: EventMask = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_InPlaceVisibilityChanged: EventMask = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_TextInserting: EventMask = 1024i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_TextInserted: EventMask = 2048i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const EventMask_All: EventMask = 4095i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACILITY_INK: u32 = 40u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_BOPOMOFO: &str = "BOPOMOFO";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_CHINESESIMPLECOMMON: &str = "CHS_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_CHINESETRADITIONALCOMMON: &str = "CHT_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_CURRENCY: &str = "CURRENCY";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_DATE: &str = "DATE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_DEFAULT: &str = "DEFAULT";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_DIGIT: &str = "DIGIT";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_EMAIL: &str = "EMAIL";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_FILENAME: &str = "FILENAME";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_HANGULCOMMON: &str = "HANGUL_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_HANGULRARE: &str = "HANGUL_RARE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_HIRAGANA: &str = "HIRAGANA";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_JAMO: &str = "JAMO";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_JAPANESECOMMON: &str = "JPN_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_KANJICOMMON: &str = "KANJI_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_KANJIRARE: &str = "KANJI_RARE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_KATAKANA: &str = "KATAKANA";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_KOREANCOMMON: &str = "KOR_COMMON";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_LOWERCHAR: &str = "LOWERCHAR";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_NONE: &str = "NONE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_NUMBER: &str = "NUMBER";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_NUMBERSIMPLE: &str = "NUMSIMPLE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_ONECHAR: &str = "ONECHAR";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_PERCENT: &str = "PERCENT";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_POSTALCODE: &str = "POSTALCODE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_PUNCCHAR: &str = "PUNCCHAR";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_SYSTEMDICTIONARY: &str = "SYSDICT";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_TELEPHONE: &str = "TELEPHONE";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_TIME: &str = "TIME";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_UPPERCHAR: &str = "UPPERCHAR";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_WEB: &str = "WEB";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FACTOID_WORDLIST: &str = "WORDLIST";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type FLICKACTION_COMMANDCODE = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKACTION_COMMANDCODE_NULL: FLICKACTION_COMMANDCODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKACTION_COMMANDCODE_SCROLL: FLICKACTION_COMMANDCODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: FLICKACTION_COMMANDCODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: FLICKACTION_COMMANDCODE = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: FLICKACTION_COMMANDCODE = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type FLICKDIRECTION = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_MIN: FLICKDIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_RIGHT: FLICKDIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_UPRIGHT: FLICKDIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_UP: FLICKDIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_UPLEFT: FLICKDIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_LEFT: FLICKDIRECTION = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_DOWNLEFT: FLICKDIRECTION = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_DOWN: FLICKDIRECTION = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_DOWNRIGHT: FLICKDIRECTION = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKDIRECTION_INVALID: FLICKDIRECTION = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type FLICKMODE = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_MIN: FLICKMODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_OFF: FLICKMODE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_ON: FLICKMODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_LEARNING: FLICKMODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_MAX: FLICKMODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICKMODE_DEFAULT: FLICKMODE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct FLICK_DATA {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_DATA {}
impl ::core::clone::Clone for FLICK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct FLICK_POINT {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_POINT {}
impl ::core::clone::Clone for FLICK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const FLICK_WM_HANDLED_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_ARROW_DOWN: u32 = 61497u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_ARROW_LEFT: u32 = 61498u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_ARROW_RIGHT: u32 = 61499u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_ARROW_UP: u32 = 61496u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_ASTERISK: u32 = 61608u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACE_LEFT: u32 = 61674u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACE_OVER: u32 = 61672u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACE_RIGHT: u32 = 61675u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACE_UNDER: u32 = 61673u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACKET_LEFT: u32 = 61670u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACKET_OVER: u32 = 61668u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACKET_RIGHT: u32 = 61671u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BRACKET_UNDER: u32 = 61669u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BULLET: u32 = 61450u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_BULLET_CROSS: u32 = 61451u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CHECK: u32 = 61445u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CHEVRON_DOWN: u32 = 61489u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CHEVRON_LEFT: u32 = 61490u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CHEVRON_RIGHT: u32 = 61491u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CHEVRON_UP: u32 = 61488u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE: u32 = 61472u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE_CIRCLE: u32 = 61475u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE_CROSS: u32 = 61477u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE_LINE_HORZ: u32 = 61479u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE_LINE_VERT: u32 = 61478u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CIRCLE_TAP: u32 = 61474u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CLOSEUP: u32 = 61455u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CROSS: u32 = 61447u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_CURLICUE: u32 = 61456u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct GESTURE_DATA {
    pub gestureId: i32,
    pub recoConfidence: i32,
    pub strokeCount: i32,
}
impl ::core::marker::Copy for GESTURE_DATA {}
impl ::core::clone::Clone for GESTURE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIAGONAL_LEFTDOWN: u32 = 61534u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIAGONAL_LEFTUP: u32 = 61532u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIAGONAL_RIGHTDOWN: u32 = 61535u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIAGONAL_RIGHTUP: u32 = 61533u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_0: u32 = 61594u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_1: u32 = 61595u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_2: u32 = 61596u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_3: u32 = 61597u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_4: u32 = 61598u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_5: u32 = 61599u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_6: u32 = 61600u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_7: u32 = 61601u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_8: u32 = 61602u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DIGIT_9: u32 = 61603u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOLLAR: u32 = 61607u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_ARROW_DOWN: u32 = 61501u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_ARROW_LEFT: u32 = 61502u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_ARROW_RIGHT: u32 = 61503u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_ARROW_UP: u32 = 61500u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_CIRCLE: u32 = 61473u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_CURLICUE: u32 = 61457u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_DOWN: u32 = 61625u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_LEFT: u32 = 61626u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_RIGHT: u32 = 61627u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_TAP: u32 = 61681u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOUBLE_UP: u32 = 61624u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN: u32 = 61529u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_ARROW_LEFT: u32 = 61506u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_ARROW_RIGHT: u32 = 61507u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_LEFT: u32 = 61546u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_LEFT_LONG: u32 = 61542u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_RIGHT: u32 = 61547u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_RIGHT_LONG: u32 = 61543u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_DOWN_UP: u32 = 61537u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_EXCLAMATION: u32 = 61604u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_INFINITY: u32 = 61446u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT: u32 = 61530u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT_ARROW_DOWN: u32 = 61509u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT_ARROW_UP: u32 = 61508u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT_DOWN: u32 = 61549u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT_RIGHT: u32 = 61538u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LEFT_UP: u32 = 61548u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_A: u32 = 61568u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_B: u32 = 61569u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_C: u32 = 61570u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_D: u32 = 61571u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_E: u32 = 61572u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_F: u32 = 61573u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_G: u32 = 61574u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_H: u32 = 61575u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_I: u32 = 61576u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_J: u32 = 61577u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_K: u32 = 61578u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_L: u32 = 61579u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_M: u32 = 61580u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_N: u32 = 61581u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_O: u32 = 61582u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_P: u32 = 61583u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_Q: u32 = 61584u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_R: u32 = 61585u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_S: u32 = 61586u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_T: u32 = 61587u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_U: u32 = 61588u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_V: u32 = 61589u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_W: u32 = 61590u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_X: u32 = 61591u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_Y: u32 = 61592u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_LETTER_Z: u32 = 61593u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_NULL: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_OPENUP: u32 = 61454u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_PARAGRAPH: u32 = 61448u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_PLUS: u32 = 61609u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_QUAD_TAP: u32 = 61683u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_QUESTION: u32 = 61605u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RECTANGLE: u32 = 61458u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT: u32 = 61531u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT_ARROW_DOWN: u32 = 61511u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT_ARROW_UP: u32 = 61510u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT_DOWN: u32 = 61551u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT_LEFT: u32 = 61539u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_RIGHT_UP: u32 = 61550u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SCRATCHOUT: u32 = 61441u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SECTION: u32 = 61449u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SEMICIRCLE_LEFT: u32 = 61480u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SEMICIRCLE_RIGHT: u32 = 61481u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SHARP: u32 = 61606u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SQUARE: u32 = 61443u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SQUIGGLE: u32 = 61452u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_STAR: u32 = 61444u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_SWAP: u32 = 61453u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TAP: u32 = 61680u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIANGLE: u32 = 61442u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIPLE_DOWN: u32 = 61629u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIPLE_LEFT: u32 = 61630u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIPLE_RIGHT: u32 = 61631u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIPLE_TAP: u32 = 61682u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_TRIPLE_UP: u32 = 61628u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP: u32 = 61528u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_ARROW_LEFT: u32 = 61504u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_ARROW_RIGHT: u32 = 61505u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_DOWN: u32 = 61536u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_LEFT: u32 = 61544u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_LEFT_LONG: u32 = 61540u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_RIGHT: u32 = 61545u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const GESTURE_UP_RIGHT_LONG: u32 = 61541u32;
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3209894802, data2: 9663, data3: 19093, data4: [137, 173, 14, 71, 107, 52, 180, 245] };
pub const GUID_GESTURE_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1105521679, data2: 9898, data3: 17754, data4: [154, 165, 44, 211, 108, 246, 63, 185] };
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195637703, data2: 63162, data3: 18694, data4: [137, 79, 102, 214, 141, 252, 69, 108] };
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 43066292, data2: 34856, data3: 16651, data4: [178, 80, 160, 83, 101, 149, 229, 220] };
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2340417476, data2: 38570, data3: 19454, data4: [172, 38, 138, 95, 11, 224, 123, 245] };
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 39345041, data2: 1179, data3: 18256, data4: [150, 21, 223, 137, 72, 171, 60, 156] };
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3875981316, data2: 22512, data3: 20224, data4: [138, 12, 133, 61, 87, 120, 155, 233] };
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3860355282, data2: 58439, data3: 16920, data4: [157, 63, 24, 134, 92, 32, 61, 244] };
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1929859117, data2: 63988, data3: 19992, data4: [179, 242, 44, 225, 177, 163, 97, 12] };
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1846413247, data2: 45031, data3: 19703, data4: [135, 209, 175, 100, 70, 32, 132, 24] };
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2138986423, data2: 48695, data3: 19425, data4: [163, 86, 122, 132, 22, 14, 24, 147] };
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1566400086, data2: 27561, data3: 19547, data4: [159, 176, 133, 28, 145, 113, 78, 86] };
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2024282966, data2: 2357, data3: 17555, data4: [186, 174, 0, 84, 26, 138, 22, 196] };
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1839483019, data2: 21060, data3: 16876, data4: [144, 91, 50, 216, 154, 184, 8, 9] };
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1130696901, data2: 65235, data3: 17873, data4: [139, 118, 113, 211, 234, 122, 130, 157] };
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 221399392, data2: 5042, data3: 16868, data4: [172, 230, 122, 233, 212, 61, 45, 59] };
pub const GUID_PACKETPROPERTY_GUID_WIDTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3131828557, data2: 10002, data3: 18677, data4: [190, 157, 143, 139, 94, 160, 113, 26] };
pub const GUID_PACKETPROPERTY_GUID_X: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1502243471, data2: 21184, data3: 19360, data4: [147, 175, 175, 53, 116, 17, 165, 97] };
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2832235322, data2: 35824, data3: 16560, data4: [149, 169, 184, 10, 107, 183, 135, 191] };
pub const GUID_PACKETPROPERTY_GUID_Y: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3040845685, data2: 1248, data3: 17560, data4: [167, 238, 195, 13, 187, 90, 144, 17] };
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1787074944, data2: 31802, data3: 17847, data4: [170, 130, 144, 162, 98, 149, 14, 137] };
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 244523913, data2: 7543, data3: 17327, data4: [172, 0, 91, 149, 13, 109, 75, 45] };
pub const GUID_PACKETPROPERTY_GUID_Z: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935334192, data2: 3771, data3: 18312, data4: [160, 228, 15, 49, 100, 144, 5, 93] };
pub const GestureRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3929065044, data2: 50732, data3: 17439, data4: [172, 0, 149, 249, 161, 150, 120, 44] };
pub type HRECOALT = isize;
pub type HRECOCONTEXT = isize;
pub type HRECOGNIZER = isize;
pub type HRECOLATTICE = isize;
pub type HRECOWORDLIST = isize;
pub const HandwrittenTextInsertion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2668056290, data2: 59113, data3: 19850, data4: [160, 71, 235, 91, 92, 60, 85, 218] };
#[repr(C)]
pub struct IDynamicRenderer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, benabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, benabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClipRectangle: unsafe extern "system" fn(this: *mut *mut Self, prccliprect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClipRectangle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClipRectangle: unsafe extern "system" fn(this: *mut *mut Self, prccliprect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClipRectangle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClipRegion: unsafe extern "system" fn(this: *mut *mut Self, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClipRegion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClipRegion: unsafe extern "system" fn(this: *mut *mut Self, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClipRegion: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, ppida: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, pida: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DrawingAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DataCacheEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DataCacheEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDataCacheEnabled: unsafe extern "system" fn(this: *mut *mut Self, fcachedata: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDataCacheEnabled: usize,
    pub ReleaseCachedData: unsafe extern "system" fn(this: *mut *mut Self, strokeid: u32) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
impl ::windows_sys::core::Interface for IDynamicRenderer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2692302478, data2: 29029, data3: 18169, data4: [183, 175, 152, 173, 1, 169, 48, 9] };
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IECN_GESTURE: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IECN_RECOGNITIONRESULT: u32 = 2051u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IECN_STROKE: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IECN__BASE: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
pub struct IEC_GESTUREINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: *mut *mut *mut *mut IInkCursor,
    pub Strokes: *mut *mut *mut *mut IInkStrokes,
    pub Gestures: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_GESTUREINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
pub struct IEC_RECOGNITIONRESULTINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub RecognitionResult: *mut *mut *mut *mut IInkRecognitionResult,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_RECOGNITIONRESULTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
pub struct IEC_STROKEINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: *mut *mut *mut *mut IInkCursor,
    pub Stroke: *mut *mut *mut *mut IInkStrokeDisp,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_STROKEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_STROKEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEC__BASE: u32 = 1536u32;
#[repr(C)]
pub struct IGestureRecognizer {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenabled: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub MaxStrokeCount: unsafe extern "system" fn(this: *mut *mut Self, pcstrokes: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxStrokeCount: unsafe extern "system" fn(this: *mut *mut Self, cstrokes: i32) -> ::windows_sys::core::HRESULT,
    pub EnableGestures: unsafe extern "system" fn(this: *mut *mut Self, cgestures: u32, pgestures: *const i32) -> ::windows_sys::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGestureRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2929653867, data2: 28756, data3: 17891, data4: [174, 34, 49, 116, 220, 136, 17, 183] };
}
#[repr(C)]
pub struct IHandwrittenTextInsertion {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InsertRecognitionResultsArray: unsafe extern "system" fn(this: *mut *mut Self, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InsertRecognitionResultsArray: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InsertInkRecognitionResult: unsafe extern "system" fn(this: *mut *mut Self, piinkrecoresult: *mut ::core::ffi::c_void, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InsertInkRecognitionResult: usize,
}
impl ::windows_sys::core::Interface for IHandwrittenTextInsertion {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1459481239, data2: 60630, data3: 17383, data4: [170, 58, 129, 107, 231, 120, 88, 96] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInk {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInk {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 66643217, data2: 17313, data3: 4563, data4: [139, 182, 0, 128, 199, 214, 186, 213] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCollector {
    pub base__: super::super::System::Com::IDispatch,
    pub hWnd: unsafe extern "system" fn(this: *mut *mut Self, currentwindow: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(this: *mut *mut Self, newwindow: isize) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, newattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Renderer: unsafe extern "system" fn(this: *mut *mut Self, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut *mut Self, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut *mut Self, newink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: i16) -> ::windows_sys::core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: *mut InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: InkMousePointer) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cursors: unsafe extern "system" fn(this: *mut *mut Self, cursors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: i32) -> ::windows_sys::core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: i16) -> ::windows_sys::core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listen: i16) -> ::windows_sys::core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut *mut Self, usemouseforinput: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut *mut Self, tablet: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCollector {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4042285237, data2: 35615, data3: 19068, data4: [137, 236, 136, 6, 146, 88, 138, 79] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCursor {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Inverted: unsafe extern "system" fn(this: *mut *mut Self, status: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, attributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, tablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tablet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Buttons: unsafe extern "system" fn(this: *mut *mut Self, buttons: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Buttons: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCursor {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2905654832, data2: 16581, data3: 17232, data4: [132, 5, 156, 113, 1, 47, 197, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCursorButton {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, currentstate: *mut InkCursorButtonState) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCursorButton {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2247070743, data2: 7513, data3: 18866, data4: [161, 60, 112, 44, 133, 67, 8, 148] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCursorButtons {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCursorButtons {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 913427520, data2: 46628, data3: 18033, data4: [159, 160, 219, 17, 157, 149, 45, 84] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCursors {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, cursor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCursors {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2722677164, data2: 50840, data3: 19974, data4: [158, 92, 213, 127, 119, 199, 230, 71] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkCustomStrokes {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkCustomStrokes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2116266127, data2: 49934, data3: 16911, data4: [155, 219, 40, 144, 37, 67, 240, 193] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDisp {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExtendedProperties: usize,
    pub Dirty: unsafe extern "system" fn(this: *mut *mut Self, dirty: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDirty: unsafe extern "system" fn(this: *mut *mut Self, dirty: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CustomStrokes: unsafe extern "system" fn(this: *mut *mut Self, ppunkinkcustomstrokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CustomStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut *mut Self, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingBox: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteStroke: unsafe extern "system" fn(this: *mut *mut Self, stroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteStroke: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExtractStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExtractStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExtractWithRectangle: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void, extractflags: InkExtractFlags, extractedink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExtractWithRectangle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clip: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, newink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HitTestCircle: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, radius: f32, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HitTestCircle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HitTestWithRectangle: unsafe extern "system" fn(this: *mut *mut Self, selectionrectangle: *mut ::core::ffi::c_void, intersectpercent: f32, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HitTestWithRectangle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub HitTestWithLasso: unsafe extern "system" fn(this: *mut *mut Self, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    HitTestWithLasso: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NearestPoint: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NearestPoint: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokeids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStrokesAtRectangle: unsafe extern "system" fn(this: *mut *mut Self, sourcestrokes: *mut ::core::ffi::c_void, targetrectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStrokesAtRectangle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Save: unsafe extern "system" fn(this: *mut *mut Self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Load: unsafe extern "system" fn(this: *mut *mut Self, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Load: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateStroke: unsafe extern "system" fn(this: *mut *mut Self, packetdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateStroke: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClipboardCopyWithRectangle: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClipboardCopyWithRectangle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClipboardCopy: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClipboardCopy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CanPaste: unsafe extern "system" fn(this: *mut *mut Self, dataobject: *mut ::core::ffi::c_void, canpaste: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CanPaste: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClipboardPaste: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, dataobject: *mut ::core::ffi::c_void, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClipboardPaste: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDisp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2637795232, data2: 50402, data3: 20429, data4: [153, 115, 151, 92, 170, 244, 126, 166] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDivider {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RecognizerContext: unsafe extern "system" fn(this: *mut *mut Self, recognizercontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecognizerContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_RecognizerContext: unsafe extern "system" fn(this: *mut *mut Self, recognizercontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_RecognizerContext: usize,
    pub LineHeight: unsafe extern "system" fn(this: *mut *mut Self, lineheight: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut *mut Self, lineheight: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Divide: unsafe extern "system" fn(this: *mut *mut Self, inkdivisionresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Divide: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDivider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1574962181, data2: 63908, data3: 18001, data4: [176, 197, 195, 23, 222, 253, 88, 185] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDivisionResult {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResultByType: unsafe extern "system" fn(this: *mut *mut Self, divisiontype: InkDivisionType, inkdivisionunits: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResultByType: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDivisionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 767475879, data2: 29895, data3: 19256, data4: [129, 235, 170, 142, 240, 194, 73, 0] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDivisionUnit {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    pub DivisionType: unsafe extern "system" fn(this: *mut *mut Self, divisiontype: *mut InkDivisionType) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RecognizedString: unsafe extern "system" fn(this: *mut *mut Self, recostring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RecognizedString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RotationTransform: unsafe extern "system" fn(this: *mut *mut Self, rotationtransform: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RotationTransform: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDivisionUnit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2242831170, data2: 18608, data3: 16964, data4: [157, 213, 30, 212, 53, 65, 15, 171] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDivisionUnits {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, inkdivisionunit: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDivisionUnits {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 464903618, data2: 12748, data3: 16693, data4: [171, 130, 44, 102, 201, 240, 12, 65] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkDrawingAttributes {
    pub base__: super::super::System::Com::IDispatch,
    pub Color: unsafe extern "system" fn(this: *mut *mut Self, currentcolor: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut *mut Self, newcolor: i32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, currentwidth: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut *mut Self, newwidth: f32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, currentheight: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut *mut Self, newheight: f32) -> ::windows_sys::core::HRESULT,
    pub FitToCurve: unsafe extern "system" fn(this: *mut *mut Self, flag: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetFitToCurve: unsafe extern "system" fn(this: *mut *mut Self, flag: i16) -> ::windows_sys::core::HRESULT,
    pub IgnorePressure: unsafe extern "system" fn(this: *mut *mut Self, flag: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetIgnorePressure: unsafe extern "system" fn(this: *mut *mut Self, flag: i16) -> ::windows_sys::core::HRESULT,
    pub AntiAliased: unsafe extern "system" fn(this: *mut *mut Self, flag: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAntiAliased: unsafe extern "system" fn(this: *mut *mut Self, flag: i16) -> ::windows_sys::core::HRESULT,
    pub Transparency: unsafe extern "system" fn(this: *mut *mut Self, currenttransparency: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTransparency: unsafe extern "system" fn(this: *mut *mut Self, newtransparency: i32) -> ::windows_sys::core::HRESULT,
    pub RasterOperation: unsafe extern "system" fn(this: *mut *mut Self, currentrasteroperation: *mut InkRasterOperation) -> ::windows_sys::core::HRESULT,
    pub SetRasterOperation: unsafe extern "system" fn(this: *mut *mut Self, newrasteroperation: InkRasterOperation) -> ::windows_sys::core::HRESULT,
    pub PenTip: unsafe extern "system" fn(this: *mut *mut Self, currentpentip: *mut InkPenTip) -> ::windows_sys::core::HRESULT,
    pub SetPenTip: unsafe extern "system" fn(this: *mut *mut Self, newpentip: InkPenTip) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExtendedProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, drawingattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkDrawingAttributes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3209796469, data2: 2581, data3: 17955, data4: [173, 201, 192, 13, 67, 106, 128, 146] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkEdit {
    pub base__: super::super::System::Com::IDispatch,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, pstatus: *mut InkEditStatus) -> ::windows_sys::core::HRESULT,
    pub UseMouseForInput: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetUseMouseForInput: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub InkMode: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut InkMode) -> ::windows_sys::core::HRESULT,
    pub SetInkMode: unsafe extern "system" fn(this: *mut *mut Self, newval: InkMode) -> ::windows_sys::core::HRESULT,
    pub InkInsertMode: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut InkInsertMode) -> ::windows_sys::core::HRESULT,
    pub SetInkInsertMode: unsafe extern "system" fn(this: *mut *mut Self, newval: InkInsertMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, newval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DrawingAttributes: usize,
    pub RecognitionTimeout: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRecognitionTimeout: unsafe extern "system" fn(this: *mut *mut Self, newval: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Recognizer: unsafe extern "system" fn(this: *mut *mut Self, newval: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Recognizer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Factoid: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Factoid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFactoid: unsafe extern "system" fn(this: *mut *mut Self, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFactoid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelInks: unsafe extern "system" fn(this: *mut *mut Self, pselink: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelInks: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelInks: unsafe extern "system" fn(this: *mut *mut Self, selink: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelInks: usize,
    pub SelInksDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, pinkdisplaymode: *mut InkDisplayMode) -> ::windows_sys::core::HRESULT,
    pub SetSelInksDisplayMode: unsafe extern "system" fn(this: *mut *mut Self, inkdisplaymode: InkDisplayMode) -> ::windows_sys::core::HRESULT,
    pub Recognize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listen: i16) -> ::windows_sys::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut *mut Self, clr: u32) -> ::windows_sys::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut *mut Self, pclr: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut *mut Self, pappearance: *mut AppearanceConstants) -> ::windows_sys::core::HRESULT,
    pub SetAppearance: unsafe extern "system" fn(this: *mut *mut Self, pappearance: AppearanceConstants) -> ::windows_sys::core::HRESULT,
    pub BorderStyle: unsafe extern "system" fn(this: *mut *mut Self, pborderstyle: *mut BorderStyleConstants) -> ::windows_sys::core::HRESULT,
    pub SetBorderStyle: unsafe extern "system" fn(this: *mut *mut Self, pborderstyle: BorderStyleConstants) -> ::windows_sys::core::HRESULT,
    pub Hwnd: unsafe extern "system" fn(this: *mut *mut Self, pohhwnd: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Font: unsafe extern "system" fn(this: *mut *mut Self, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Font: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Font: unsafe extern "system" fn(this: *mut *mut Self, ppfont: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Font: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Text: unsafe extern "system" fn(this: *mut *mut Self, pbstrtext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Text: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetText: unsafe extern "system" fn(this: *mut *mut Self, pbstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetText: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: *mut InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub Locked: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetLocked: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut *mut Self, plmaxlength: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMaxLength: unsafe extern "system" fn(this: *mut *mut Self, lmaxlength: i32) -> ::windows_sys::core::HRESULT,
    pub MultiLine: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetMultiLine: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    pub ScrollBars: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut ScrollBarsConstants) -> ::windows_sys::core::HRESULT,
    pub SetScrollBars: unsafe extern "system" fn(this: *mut *mut Self, newval: ScrollBarsConstants) -> ::windows_sys::core::HRESULT,
    pub DisableNoScroll: unsafe extern "system" fn(this: *mut *mut Self, pval: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDisableNoScroll: unsafe extern "system" fn(this: *mut *mut Self, newval: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelAlignment: unsafe extern "system" fn(this: *mut *mut Self, pvarselalignment: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelAlignment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelAlignment: unsafe extern "system" fn(this: *mut *mut Self, pvarselalignment: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelAlignment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelBold: unsafe extern "system" fn(this: *mut *mut Self, pvarselbold: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelBold: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelBold: unsafe extern "system" fn(this: *mut *mut Self, pvarselbold: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelBold: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelItalic: unsafe extern "system" fn(this: *mut *mut Self, pvarselitalic: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelItalic: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelItalic: unsafe extern "system" fn(this: *mut *mut Self, pvarselitalic: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelItalic: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelUnderline: unsafe extern "system" fn(this: *mut *mut Self, pvarselunderline: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelUnderline: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelUnderline: unsafe extern "system" fn(this: *mut *mut Self, pvarselunderline: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelUnderline: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelColor: unsafe extern "system" fn(this: *mut *mut Self, pvarselcolor: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelColor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelColor: unsafe extern "system" fn(this: *mut *mut Self, pvarselcolor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelColor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelFontName: unsafe extern "system" fn(this: *mut *mut Self, pvarselfontname: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelFontName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelFontName: unsafe extern "system" fn(this: *mut *mut Self, pvarselfontname: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelFontName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelFontSize: unsafe extern "system" fn(this: *mut *mut Self, pvarselfontsize: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelFontSize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelFontSize: unsafe extern "system" fn(this: *mut *mut Self, pvarselfontsize: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelFontSize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelCharOffset: unsafe extern "system" fn(this: *mut *mut Self, pvarselcharoffset: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelCharOffset: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSelCharOffset: unsafe extern "system" fn(this: *mut *mut Self, pvarselcharoffset: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSelCharOffset: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TextRTF: unsafe extern "system" fn(this: *mut *mut Self, pbstrtextrtf: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TextRTF: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextRTF: unsafe extern "system" fn(this: *mut *mut Self, pbstrtextrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextRTF: usize,
    pub SelStart: unsafe extern "system" fn(this: *mut *mut Self, plselstart: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelStart: unsafe extern "system" fn(this: *mut *mut Self, plselstart: i32) -> ::windows_sys::core::HRESULT,
    pub SelLength: unsafe extern "system" fn(this: *mut *mut Self, plsellength: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetSelLength: unsafe extern "system" fn(this: *mut *mut Self, plsellength: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SelText: unsafe extern "system" fn(this: *mut *mut Self, pbstrseltext: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelText: unsafe extern "system" fn(this: *mut *mut Self, pbstrseltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SelRTF: unsafe extern "system" fn(this: *mut *mut Self, pbstrselrtf: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelRTF: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelRTF: unsafe extern "system" fn(this: *mut *mut Self, pbstrselrtf: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelRTF: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkEdit {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4061297177, data2: 64507, data3: 19181, data4: [132, 100, 63, 54, 215, 140, 254, 251] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkExtendedProperties {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, identifier: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesPropertyExist: unsafe extern "system" fn(this: *mut *mut Self, guid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesPropertyExist: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkExtendedProperties {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2314381502, data2: 38313, data3: 17712, data4: [139, 143, 136, 233, 113, 227, 226, 95] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkExtendedProperty {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Guid: unsafe extern "system" fn(this: *mut *mut Self, guid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Guid: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, data: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Data: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, data: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkExtendedProperty {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3678966281, data2: 47043, data3: 16669, data4: [144, 246, 21, 72, 207, 255, 39, 30] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkGesture {
    pub base__: super::super::System::Com::IDispatch,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, confidence: *mut InkRecognitionConfidence) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, id: *mut InkApplicationGesture) -> ::windows_sys::core::HRESULT,
    pub GetHotPoint: unsafe extern "system" fn(this: *mut *mut Self, x: *mut i32, y: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkGesture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1004276375, data2: 1253, data3: 20006, data4: [184, 19, 24, 240, 82, 212, 29, 239] };
}
#[repr(C)]
pub struct IInkLineInfo {
    pub base__: ::windows_sys::core::IUnknown,
    pub SetFormat: unsafe extern "system" fn(this: *mut *mut Self, pim: *const INKMETRIC) -> ::windows_sys::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut *mut Self, pim: *const INKMETRIC) -> ::windows_sys::core::HRESULT,
    pub GetInkExtent: unsafe extern "system" fn(this: *mut *mut Self, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows_sys::core::HRESULT,
    pub GetCandidate: unsafe extern "system" fn(this: *mut *mut Self, ncandidatenum: u32, pwcrecogword: ::windows_sys::core::PCWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows_sys::core::HRESULT,
    pub SetCandidate: unsafe extern "system" fn(this: *mut *mut Self, ncandidatenum: u32, strrecogword: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT,
    pub Recognize: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInkLineInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2619103958, data2: 61999, data3: 19940, data4: [180, 83, 162, 204, 72, 46, 124, 51] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkOverlay {
    pub base__: super::super::System::Com::IDispatch,
    pub hWnd: unsafe extern "system" fn(this: *mut *mut Self, currentwindow: *mut isize) -> ::windows_sys::core::HRESULT,
    pub SethWnd: unsafe extern "system" fn(this: *mut *mut Self, newwindow: isize) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, newattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Renderer: unsafe extern "system" fn(this: *mut *mut Self, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut *mut Self, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut *mut Self, newink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: i16) -> ::windows_sys::core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: *mut InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub EditingMode: unsafe extern "system" fn(this: *mut *mut Self, editingmode: *mut InkOverlayEditingMode) -> ::windows_sys::core::HRESULT,
    pub SetEditingMode: unsafe extern "system" fn(this: *mut *mut Self, editingmode: InkOverlayEditingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, selection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Selection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, selection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSelection: usize,
    pub EraserMode: unsafe extern "system" fn(this: *mut *mut Self, erasermode: *mut InkOverlayEraserMode) -> ::windows_sys::core::HRESULT,
    pub SetEraserMode: unsafe extern "system" fn(this: *mut *mut Self, erasermode: InkOverlayEraserMode) -> ::windows_sys::core::HRESULT,
    pub EraserWidth: unsafe extern "system" fn(this: *mut *mut Self, eraserwidth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEraserWidth: unsafe extern "system" fn(this: *mut *mut Self, neweraserwidth: i32) -> ::windows_sys::core::HRESULT,
    pub AttachMode: unsafe extern "system" fn(this: *mut *mut Self, attachmode: *mut InkOverlayAttachMode) -> ::windows_sys::core::HRESULT,
    pub SetAttachMode: unsafe extern "system" fn(this: *mut *mut Self, attachmode: InkOverlayAttachMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cursors: unsafe extern "system" fn(this: *mut *mut Self, cursors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: i32) -> ::windows_sys::core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: i16) -> ::windows_sys::core::HRESULT,
    pub SupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut *mut Self, support: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut *mut Self, support: i16) -> ::windows_sys::core::HRESULT,
    pub HitTestSelection: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, rect: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Draw: usize,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listen: i16) -> ::windows_sys::core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut *mut Self, usemouseforinput: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut *mut Self, tablet: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkOverlay {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3089778235, data2: 49605, data3: 17827, data4: [153, 124, 222, 171, 86, 81, 182, 122] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkPicture {
    pub base__: super::super::System::Com::IDispatch,
    pub hWnd: unsafe extern "system" fn(this: *mut *mut Self, currentwindow: *mut isize) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, currentattributes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DefaultDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, newattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DefaultDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Renderer: unsafe extern "system" fn(this: *mut *mut Self, currentinkrenderer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Renderer: unsafe extern "system" fn(this: *mut *mut Self, newinkrenderer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Renderer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut *mut Self, newink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Ink: usize,
    pub AutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoRedraw: unsafe extern "system" fn(this: *mut *mut Self, autoredraw: i16) -> ::windows_sys::core::HRESULT,
    pub CollectingInk: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub CollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub SetCollectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: InkCollectionMode) -> ::windows_sys::core::HRESULT,
    pub DynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetDynamicRendering: unsafe extern "system" fn(this: *mut *mut Self, enabled: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetguids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDesiredPacketDescription: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetMouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetMouseIcon: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_MouseIcon: unsafe extern "system" fn(this: *mut *mut Self, mouseicon: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_MouseIcon: usize,
    pub MousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: *mut InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub SetMousePointer: unsafe extern "system" fn(this: *mut *mut Self, mousepointer: InkMousePointer) -> ::windows_sys::core::HRESULT,
    pub EditingMode: unsafe extern "system" fn(this: *mut *mut Self, editingmode: *mut InkOverlayEditingMode) -> ::windows_sys::core::HRESULT,
    pub SetEditingMode: unsafe extern "system" fn(this: *mut *mut Self, editingmode: InkOverlayEditingMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Selection: unsafe extern "system" fn(this: *mut *mut Self, selection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Selection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSelection: unsafe extern "system" fn(this: *mut *mut Self, selection: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSelection: usize,
    pub EraserMode: unsafe extern "system" fn(this: *mut *mut Self, erasermode: *mut InkOverlayEraserMode) -> ::windows_sys::core::HRESULT,
    pub SetEraserMode: unsafe extern "system" fn(this: *mut *mut Self, erasermode: InkOverlayEraserMode) -> ::windows_sys::core::HRESULT,
    pub EraserWidth: unsafe extern "system" fn(this: *mut *mut Self, eraserwidth: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetEraserWidth: unsafe extern "system" fn(this: *mut *mut Self, neweraserwidth: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub putref_Picture: unsafe extern "system" fn(this: *mut *mut Self, ppicture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    putref_Picture: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPicture: unsafe extern "system" fn(this: *mut *mut Self, ppicture: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPicture: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Picture: unsafe extern "system" fn(this: *mut *mut Self, pppicture: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Picture: usize,
    pub SetSizeMode: unsafe extern "system" fn(this: *mut *mut Self, smnewsizemode: InkPictureSizeMode) -> ::windows_sys::core::HRESULT,
    pub SizeMode: unsafe extern "system" fn(this: *mut *mut Self, smsizemode: *mut InkPictureSizeMode) -> ::windows_sys::core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut *mut Self, newcolor: u32) -> ::windows_sys::core::HRESULT,
    pub BackColor: unsafe extern "system" fn(this: *mut *mut Self, pcolor: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Cursors: unsafe extern "system" fn(this: *mut *mut Self, cursors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cursors: usize,
    pub MarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginX: unsafe extern "system" fn(this: *mut *mut Self, marginx: i32) -> ::windows_sys::core::HRESULT,
    pub MarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMarginY: unsafe extern "system" fn(this: *mut *mut Self, marginy: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, singletablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tablet: usize,
    pub SupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSupportHighContrastInk: unsafe extern "system" fn(this: *mut *mut Self, support: i16) -> ::windows_sys::core::HRESULT,
    pub SupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut *mut Self, support: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetSupportHighContrastSelectionUI: unsafe extern "system" fn(this: *mut *mut Self, support: i16) -> ::windows_sys::core::HRESULT,
    pub HitTestSelection: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows_sys::core::HRESULT,
    pub SetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listen: i16) -> ::windows_sys::core::HRESULT,
    pub GetGestureStatus: unsafe extern "system" fn(this: *mut *mut Self, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWindowInputRectangle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, windowinputrectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWindowInputRectangle: usize,
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut *mut Self, usemouseforinput: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSingleTabletIntegratedMode: unsafe extern "system" fn(this: *mut *mut Self, tablet: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSingleTabletIntegratedMode: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(this: *mut *mut Self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows_sys::core::HRESULT,
    pub InkEnabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetInkEnabled: unsafe extern "system" fn(this: *mut *mut Self, collecting: i16) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pbool: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, vbool: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkPicture {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3897975520, data2: 14234, data3: 16599, data4: [155, 92, 117, 125, 35, 63, 153, 35] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognitionAlternate {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub String: unsafe extern "system" fn(this: *mut *mut Self, recostring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    String: usize,
    pub Confidence: unsafe extern "system" fn(this: *mut *mut Self, confidence: *mut InkRecognitionConfidence) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Baseline: unsafe extern "system" fn(this: *mut *mut Self, baseline: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Baseline: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Midline: unsafe extern "system" fn(this: *mut *mut Self, midline: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Midline: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Ascender: unsafe extern "system" fn(this: *mut *mut Self, ascender: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Ascender: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Descender: unsafe extern "system" fn(this: *mut *mut Self, descender: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Descender: usize,
    pub LineNumber: unsafe extern "system" fn(this: *mut *mut Self, linenumber: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LineAlternates: unsafe extern "system" fn(this: *mut *mut Self, linealternates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LineAlternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ConfidenceAlternates: unsafe extern "system" fn(this: *mut *mut Self, confidencealternates: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConfidenceAlternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStrokesFromStrokeRanges: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, getstrokesfromstrokeranges: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStrokesFromStrokeRanges: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStrokesFromTextRange: unsafe extern "system" fn(this: *mut *mut Self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStrokesFromTextRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTextRangeFromStrokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTextRangeFromStrokes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AlternatesWithConstantPropertyValues: unsafe extern "system" fn(this: *mut *mut Self, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AlternatesWithConstantPropertyValues: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut *mut Self, propertytype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPropertyValue: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognitionAlternate {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085328557, data2: 30692, data3: 17051, data4: [173, 218, 135, 55, 128, 209, 252, 74] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognitionAlternates {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, inkrecoalternate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognitionAlternates {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 678041215, data2: 40729, data3: 19553, data4: [157, 83, 79, 7, 190, 98, 43, 132] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognitionResult {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub TopString: unsafe extern "system" fn(this: *mut *mut Self, topstring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TopString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TopAlternate: unsafe extern "system" fn(this: *mut *mut Self, topalternate: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TopAlternate: usize,
    pub TopConfidence: unsafe extern "system" fn(this: *mut *mut Self, topconfidence: *mut InkRecognitionConfidence) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AlternatesFromSelection: unsafe extern "system" fn(this: *mut *mut Self, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AlternatesFromSelection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyTopAlternate: unsafe extern "system" fn(this: *mut *mut Self, alternate: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyTopAlternate: usize,
    pub SetResultOnStrokes: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognitionResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1002514856, data2: 34509, data3: 17837, data4: [189, 232, 224, 211, 45, 97, 193, 109] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizer {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Vendor: unsafe extern "system" fn(this: *mut *mut Self, vendor: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Vendor: usize,
    pub Capabilities: unsafe extern "system" fn(this: *mut *mut Self, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Languages: unsafe extern "system" fn(this: *mut *mut Self, languages: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Languages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SupportedProperties: unsafe extern "system" fn(this: *mut *mut Self, supportedproperties: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SupportedProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PreferredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, preferredpacketdescription: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PreferredPacketDescription: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRecognizerContext: unsafe extern "system" fn(this: *mut *mut Self, context: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRecognizerContext: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2016147407, data2: 843, data3: 17302, data4: [138, 50, 58, 24, 51, 207, 107, 86] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizer2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UnicodeRanges: unsafe extern "system" fn(this: *mut *mut Self, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UnicodeRanges: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizer2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1628443018, data2: 14965, data3: 19158, data4: [178, 170, 4, 178, 183, 43, 190, 101] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizerContext {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Strokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Strokes: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Strokes: usize,
    pub CharacterAutoCompletionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows_sys::core::HRESULT,
    pub SetCharacterAutoCompletionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Factoid: unsafe extern "system" fn(this: *mut *mut Self, factoid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Factoid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFactoid: unsafe extern "system" fn(this: *mut *mut Self, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFactoid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Guide: unsafe extern "system" fn(this: *mut *mut Self, recognizerguide: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Guide: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Guide: unsafe extern "system" fn(this: *mut *mut Self, recognizerguide: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Guide: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PrefixText: unsafe extern "system" fn(this: *mut *mut Self, prefix: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrefixText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPrefixText: unsafe extern "system" fn(this: *mut *mut Self, prefix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPrefixText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SuffixText: unsafe extern "system" fn(this: *mut *mut Self, suffix: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SuffixText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSuffixText: unsafe extern "system" fn(this: *mut *mut Self, suffix: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSuffixText: usize,
    pub RecognitionFlags: unsafe extern "system" fn(this: *mut *mut Self, modes: *mut InkRecognitionModes) -> ::windows_sys::core::HRESULT,
    pub SetRecognitionFlags: unsafe extern "system" fn(this: *mut *mut Self, modes: InkRecognitionModes) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WordList: unsafe extern "system" fn(this: *mut *mut Self, wordlist: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WordList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_WordList: unsafe extern "system" fn(this: *mut *mut Self, wordlist: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_WordList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut *mut Self, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognize: unsafe extern "system" fn(this: *mut *mut Self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognize: usize,
    pub StopBackgroundRecognition: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EndInkInput: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BackgroundRecognize: unsafe extern "system" fn(this: *mut *mut Self, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BackgroundRecognize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BackgroundRecognizeWithAlternates: unsafe extern "system" fn(this: *mut *mut Self, customdata: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BackgroundRecognizeWithAlternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStringSupported: unsafe extern "system" fn(this: *mut *mut Self, string: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStringSupported: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizerContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3331281657, data2: 12963, data3: 17957, data4: [144, 108, 68, 252, 35, 180, 9, 88] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizerContext2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnabledUnicodeRanges: unsafe extern "system" fn(this: *mut *mut Self, unicoderanges: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnabledUnicodeRanges: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetEnabledUnicodeRanges: unsafe extern "system" fn(this: *mut *mut Self, unicoderanges: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetEnabledUnicodeRanges: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizerContext2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3606111023, data2: 29656, data3: 16526, data4: [142, 159, 95, 234, 89, 44, 54, 63] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizerGuide {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub WritingBox: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WritingBox: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWritingBox: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWritingBox: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawnBox: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawnBox: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDrawnBox: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDrawnBox: usize,
    pub Rows: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRows: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    pub Columns: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetColumns: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    pub Midline: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetMidline: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GuideData: unsafe extern "system" fn(this: *mut *mut Self, precoguide: *mut InkRecoGuide) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GuideData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGuideData: unsafe extern "system" fn(this: *mut *mut Self, recoguide: InkRecoGuide) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGuideData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizerGuide {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3644112391, data2: 31620, data3: 16904, data4: [145, 54, 131, 194, 9, 148, 233, 5] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRecognizers {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDefaultRecognizer: unsafe extern "system" fn(this: *mut *mut Self, lcid: i32, defaultrecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDefaultRecognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, inkrecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRecognizers {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2630635282, data2: 45239, data3: 19083, data4: [191, 88, 74, 236, 164, 232, 206, 253] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRectangle {
    pub base__: super::super::System::Com::IDispatch,
    pub Top: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    pub Bottom: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    pub Right: unsafe extern "system" fn(this: *mut *mut Self, units: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut *mut Self, units: i32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, rect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, rect: super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
    pub GetRectangle: unsafe extern "system" fn(this: *mut *mut Self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetRectangle: unsafe extern "system" fn(this: *mut *mut Self, top: i32, left: i32, bottom: i32, right: i32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRectangle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2543124354, data2: 24689, data3: 18199, data4: [138, 139, 106, 199, 198, 74, 104, 110] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkRenderer {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_System_Com")]
    pub GetViewTransform: unsafe extern "system" fn(this: *mut *mut Self, viewtransform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetViewTransform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetViewTransform: unsafe extern "system" fn(this: *mut *mut Self, viewtransform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetViewTransform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObjectTransform: unsafe extern "system" fn(this: *mut *mut Self, objecttransform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObjectTransform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetObjectTransform: unsafe extern "system" fn(this: *mut *mut Self, objecttransform: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetObjectTransform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Draw: unsafe extern "system" fn(this: *mut *mut Self, hdc: isize, strokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Draw: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawStroke: unsafe extern "system" fn(this: *mut *mut Self, hdc: isize, stroke: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawStroke: usize,
    pub PixelToInkSpace: unsafe extern "system" fn(this: *mut *mut Self, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows_sys::core::HRESULT,
    pub InkSpaceToPixel: unsafe extern "system" fn(this: *mut *mut Self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PixelToInkSpaceFromPoints: unsafe extern "system" fn(this: *mut *mut Self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PixelToInkSpaceFromPoints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InkSpaceToPixelFromPoints: unsafe extern "system" fn(this: *mut *mut Self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InkSpaceToPixelFromPoints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Measure: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Measure: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MeasureStroke: unsafe extern "system" fn(this: *mut *mut Self, stroke: *mut ::core::ffi::c_void, drawingattributes: *mut ::core::ffi::c_void, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MeasureStroke: usize,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_sys::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut *mut Self, degrees: f32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkRenderer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3861215900, data2: 46353, data3: 20300, data4: [168, 176, 167, 219, 201, 80, 107, 131] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkStrokeDisp {
    pub base__: super::super::System::Com::IDispatch,
    pub ID: unsafe extern "system" fn(this: *mut *mut Self, id: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BezierPoints: unsafe extern "system" fn(this: *mut *mut Self, points: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BezierPoints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, drawattrs: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, drawattrs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_DrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut *mut Self, properties: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExtendedProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolylineCusps: unsafe extern "system" fn(this: *mut *mut Self, cusps: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolylineCusps: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BezierCusps: unsafe extern "system" fn(this: *mut *mut Self, cusps: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BezierCusps: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SelfIntersections: unsafe extern "system" fn(this: *mut *mut Self, intersections: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SelfIntersections: usize,
    pub PacketCount: unsafe extern "system" fn(this: *mut *mut Self, plcount: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PacketSize: unsafe extern "system" fn(this: *mut *mut Self, plsize: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PacketDescription: unsafe extern "system" fn(this: *mut *mut Self, packetdescription: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PacketDescription: usize,
    pub Deleted: unsafe extern "system" fn(this: *mut *mut Self, deleted: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut *mut Self, boundingboxmode: InkBoundingBoxMode, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingBox: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindIntersections: unsafe extern "system" fn(this: *mut *mut Self, strokes: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindIntersections: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRectangleIntersections: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void, intersections: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRectangleIntersections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clip: usize,
    pub HitTestCircle: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows_sys::core::HRESULT,
    pub NearestPoint: unsafe extern "system" fn(this: *mut *mut Self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Split: unsafe extern "system" fn(this: *mut *mut Self, splitat: f32, newstroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Split: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPacketDescriptionPropertyMetrics: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPacketDescriptionPropertyMetrics: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPoints: unsafe extern "system" fn(this: *mut *mut Self, index: i32, count: i32, points: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPoints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, points: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPoints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPacketData: unsafe extern "system" fn(this: *mut *mut Self, index: i32, count: i32, packetdata: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPacketData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPacketValuesByProperty: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPacketValuesByProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetPacketValuesByProperty: unsafe extern "system" fn(this: *mut *mut Self, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetPacketValuesByProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFlattenedBezierPoints: unsafe extern "system" fn(this: *mut *mut Self, fittingerror: i32, flattenedbezierpoints: *mut super::super::System::Com::VARIANT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFlattenedBezierPoints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void, applyonpenwidth: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ScaleToRectangle: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScaleToRectangle: usize,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_sys::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut *mut Self, degrees: f32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_sys::core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkStrokeDisp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1126445034, data2: 37329, data3: 19058, data4: [150, 62, 251, 185, 24, 41, 207, 162] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkStrokes {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RecognitionResult: unsafe extern "system" fn(this: *mut *mut Self, recognitionresult: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecognitionResult: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ToString: unsafe extern "system" fn(this: *mut *mut Self, tostring: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ToString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, stroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut *mut Self, inkstroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddStrokes: unsafe extern "system" fn(this: *mut *mut Self, inkstrokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, inkstroke: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveStrokes: unsafe extern "system" fn(this: *mut *mut Self, inkstrokes: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveStrokes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifyDrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, drawattrs: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifyDrawingAttributes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBoundingBox: unsafe extern "system" fn(this: *mut *mut Self, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBoundingBox: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transform: unsafe extern "system" fn(this: *mut *mut Self, transform: *mut ::core::ffi::c_void, applyonpenwidth: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transform: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ScaleToRectangle: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScaleToRectangle: usize,
    pub Move: unsafe extern "system" fn(this: *mut *mut Self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_sys::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut *mut Self, degrees: f32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_sys::core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clip: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clip: usize,
    pub RemoveRecognitionResult: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkStrokes {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4059351512, data2: 22794, data3: 18787, data4: [179, 174, 25, 53, 103, 27, 182, 243] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkTablet {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, name: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PlugAndPlayId: unsafe extern "system" fn(this: *mut *mut Self, id: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PlugAndPlayId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MaximumInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, rectangle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MaximumInputRectangle: usize,
    pub HardwareCapabilities: unsafe extern "system" fn(this: *mut *mut Self, capabilities: *mut TabletHardwareCapabilities) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPacketPropertySupported: unsafe extern "system" fn(this: *mut *mut Self, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPacketPropertySupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyMetrics: unsafe extern "system" fn(this: *mut *mut Self, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyMetrics: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkTablet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 769810090, data2: 28408, data3: 17109, data4: [174, 233, 24, 91, 200, 27, 145, 45] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkTablet2 {
    pub base__: super::super::System::Com::IDispatch,
    pub DeviceKind: unsafe extern "system" fn(this: *mut *mut Self, kind: *mut TabletDeviceKind) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkTablet2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2429098706, data2: 64054, data3: 18902, data4: [149, 22, 206, 141, 87, 15, 111, 133] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkTablet3 {
    pub base__: super::super::System::Com::IDispatch,
    pub IsMultiTouch: unsafe extern "system" fn(this: *mut *mut Self, pismultitouch: *mut i16) -> ::windows_sys::core::HRESULT,
    pub MaximumCursors: unsafe extern "system" fn(this: *mut *mut Self, pmaximumcursors: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkTablet3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2117155223, data2: 4903, data3: 16861, data4: [140, 169, 121, 242, 75, 225, 114, 80] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkTablets {
    pub base__: super::super::System::Com::IDispatch,
    pub Count: unsafe extern "system" fn(this: *mut *mut Self, count: *mut i32) -> ::windows_sys::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut *mut Self, _newenum: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultTablet: unsafe extern "system" fn(this: *mut *mut Self, defaulttablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultTablet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut *mut Self, index: i32, tablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPacketPropertySupported: unsafe extern "system" fn(this: *mut *mut Self, packetpropertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPacketPropertySupported: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkTablets {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 287344345, data2: 30585, data3: 17717, data4: [166, 153, 134, 43, 67, 172, 24, 99] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkTransform {
    pub base__: super::super::System::Com::IDispatch,
    pub Reset: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Translate: unsafe extern "system" fn(this: *mut *mut Self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_sys::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut *mut Self, degrees: f32, x: f32, y: f32) -> ::windows_sys::core::HRESULT,
    pub Reflect: unsafe extern "system" fn(this: *mut *mut Self, horizontally: i16, vertically: i16) -> ::windows_sys::core::HRESULT,
    pub Shear: unsafe extern "system" fn(this: *mut *mut Self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows_sys::core::HRESULT,
    pub ScaleTransform: unsafe extern "system" fn(this: *mut *mut Self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows_sys::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut *mut Self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut *mut Self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows_sys::core::HRESULT,
    pub eM11: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteM11: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub eM12: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteM12: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub eM21: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteM21: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub eM22: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteM22: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub eDx: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteDx: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    pub eDy: unsafe extern "system" fn(this: *mut *mut Self, value: *mut f32) -> ::windows_sys::core::HRESULT,
    pub SeteDy: unsafe extern "system" fn(this: *mut *mut Self, value: f32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Data: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, xform: super::super::Graphics::Gdi::XFORM) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetData: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkTransform {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1633623363, data2: 34563, data3: 17765, data4: [136, 226, 130, 1, 210, 236, 215, 183] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkWordList {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWord: unsafe extern "system" fn(this: *mut *mut Self, newword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWord: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveWord: unsafe extern "system" fn(this: *mut *mut Self, removeword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveWord: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Merge: unsafe extern "system" fn(this: *mut *mut Self, mergewordlist: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Merge: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkWordList {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1991914641, data2: 52015, data3: 16491, data4: [153, 97, 14, 12, 76, 218, 174, 242] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInkWordList2 {
    pub base__: super::super::System::Com::IDispatch,
    #[cfg(feature = "Win32_Foundation")]
    pub AddWords: unsafe extern "system" fn(this: *mut *mut Self, newwords: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddWords: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IInkWordList2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 341058950, data2: 4543, data3: 20319, data4: [182, 231, 73, 208, 116, 74, 171, 110] };
}
#[repr(C)]
pub struct IInputPanelWindowHandle {
    pub base__: ::windows_sys::core::IUnknown,
    pub AttachedEditWindow32: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAttachedEditWindow32: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: i32) -> ::windows_sys::core::HRESULT,
    pub AttachedEditWindow64: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetAttachedEditWindow64: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: i64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IInputPanelWindowHandle {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1257773127, data2: 64964, data3: 20419, data4: [173, 11, 66, 36, 121, 193, 185, 53] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMathInputControl {
    pub base__: super::super::System::Com::IDispatch,
    pub Show: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut *mut Self, pvbshown: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut *mut Self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut *mut Self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetCustomPaint: unsafe extern "system" fn(this: *mut *mut Self, element: i32, paint: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCaptionText: unsafe extern "system" fn(this: *mut *mut Self, captiontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCaptionText: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadInk: unsafe extern "system" fn(this: *mut *mut Self, ink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadInk: usize,
    pub SetOwnerWindow: unsafe extern "system" fn(this: *mut *mut Self, ownerwindow: isize) -> ::windows_sys::core::HRESULT,
    pub EnableExtendedButtons: unsafe extern "system" fn(this: *mut *mut Self, extended: i16) -> ::windows_sys::core::HRESULT,
    pub GetPreviewHeight: unsafe extern "system" fn(this: *mut *mut Self, height: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetPreviewHeight: unsafe extern "system" fn(this: *mut *mut Self, height: i32) -> ::windows_sys::core::HRESULT,
    pub EnableAutoGrow: unsafe extern "system" fn(this: *mut *mut Self, autogrow: i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddFunctionName: unsafe extern "system" fn(this: *mut *mut Self, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddFunctionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveFunctionName: unsafe extern "system" fn(this: *mut *mut Self, functionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveFunctionName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetHoverIcon: unsafe extern "system" fn(this: *mut *mut Self, hoverimage: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetHoverIcon: usize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IMathInputControl {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3953530282, data2: 64198, data3: 18232, data4: [186, 95, 255, 9, 233, 254, 71, 62] };
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKEDIT_CLASS: &str = "INKEDIT";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKEDIT_CLASSW: &str = "INKEDIT";
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct INKMETRIC {
    pub iHeight: i32,
    pub iFontAscent: i32,
    pub iFontDescent: i32,
    pub dwFlags: u32,
    pub color: u32,
}
impl ::core::marker::Copy for INKMETRIC {}
impl ::core::clone::Clone for INKMETRIC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_BOXNUMBER: &str = "{2C243E3A-F733-4EB6-B1F8-B5DC5C2C4CDA}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_CONFIDENCELEVEL: &str = "{7DFE11A7-FB5D-4958-8765-154ADF0D833F}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_HOTPOINT: &str = "{CA6F40DC-5292-452a-91FB-2181C0BEC0DE}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_LINEMETRICS: &str = "{8CC24B27-30A9-4b96-9056-2D3A90DA0727}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_LINENUMBER: &str = "{DBF29F2C-5289-4BE8-B3D8-6EF63246253E}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_MAXIMUMSTROKECOUNT: &str = "{BF0EEC4E-4B7D-47a9-8CFA-234DD24BD22A}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_POINTSPERINCH: &str = "{7ED16B76-889C-468e-8276-0021B770187E}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INKRECOGNITIONPROPERTY_SEGMENTATION: &str = "{B3C0FE6C-FB51-4164-BA2F-844AF8F983DA}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const INK_SERIALIZED_FORMAT: &str = "Ink Serialized Format";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IP_CURSOR_DOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IP_INVERTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IP_MARGIN: u32 = 4u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPenInputPanel {
    pub base__: super::super::System::Com::IDispatch,
    pub Busy: unsafe extern "system" fn(this: *mut *mut Self, busy: *mut i16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Factoid: unsafe extern "system" fn(this: *mut *mut Self, factoid: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Factoid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFactoid: unsafe extern "system" fn(this: *mut *mut Self, factoid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFactoid: usize,
    pub AttachedEditWindow: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetAttachedEditWindow: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: i32) -> ::windows_sys::core::HRESULT,
    pub CurrentPanel: unsafe extern "system" fn(this: *mut *mut Self, currentpanel: *mut PanelType) -> ::windows_sys::core::HRESULT,
    pub SetCurrentPanel: unsafe extern "system" fn(this: *mut *mut Self, currentpanel: PanelType) -> ::windows_sys::core::HRESULT,
    pub DefaultPanel: unsafe extern "system" fn(this: *mut *mut Self, pdefaultpanel: *mut PanelType) -> ::windows_sys::core::HRESULT,
    pub SetDefaultPanel: unsafe extern "system" fn(this: *mut *mut Self, defaultpanel: PanelType) -> ::windows_sys::core::HRESULT,
    pub Visible: unsafe extern "system" fn(this: *mut *mut Self, visible: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut *mut Self, visible: i16) -> ::windows_sys::core::HRESULT,
    pub Top: unsafe extern "system" fn(this: *mut *mut Self, top: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut *mut Self, left: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, width: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, height: *mut i32) -> ::windows_sys::core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, verticaloffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut *mut Self, verticaloffset: i32) -> ::windows_sys::core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, horizontaloffset: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut *mut Self, horizontaloffset: i32) -> ::windows_sys::core::HRESULT,
    pub AutoShow: unsafe extern "system" fn(this: *mut *mut Self, pautoshow: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetAutoShow: unsafe extern "system" fn(this: *mut *mut Self, autoshow: i16) -> ::windows_sys::core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(this: *mut *mut Self, left: i32, top: i32) -> ::windows_sys::core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub EnableTsf: unsafe extern "system" fn(this: *mut *mut Self, enable: i16) -> ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for IPenInputPanel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4202315907, data2: 22343, data3: 16448, data4: [161, 130, 11, 14, 159, 212, 250, 199] };
}
#[repr(C)]
pub struct IRealTimeStylus {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HWND: unsafe extern "system" fn(this: *mut *mut Self, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHWND: unsafe extern "system" fn(this: *mut *mut Self, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WindowInputRectangle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowInputRectangle: unsafe extern "system" fn(this: *mut *mut Self, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowInputRectangle: usize,
    pub AddStylusSyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, piplugin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveStylusSyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAllStylusSyncPlugins: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetStylusSyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylusSyncPluginCount: unsafe extern "system" fn(this: *mut *mut Self, pcplugins: *mut u32) -> ::windows_sys::core::HRESULT,
    pub AddStylusAsyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, piplugin: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveStylusAsyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAllStylusAsyncPlugins: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetStylusAsyncPlugin: unsafe extern "system" fn(this: *mut *mut Self, iindex: u32, ppiplugin: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStylusAsyncPluginCount: unsafe extern "system" fn(this: *mut *mut Self, pcplugins: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ChildRealTimeStylusPlugin: unsafe extern "system" fn(this: *mut *mut Self, ppirts: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub putref_ChildRealTimeStylusPlugin: unsafe extern "system" fn(this: *mut *mut Self, pirts: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddCustomStylusDataToQueue: unsafe extern "system" fn(this: *mut *mut Self, sq: StylusQueue, pguidid: *const ::windows_sys::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows_sys::core::HRESULT,
    pub ClearStylusQueues: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllTabletsMode: unsafe extern "system" fn(this: *mut *mut Self, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllTabletsMode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSingleTabletMode: unsafe extern "system" fn(this: *mut *mut Self, pitablet: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSingleTabletMode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTablet: unsafe extern "system" fn(this: *mut *mut Self, ppisingletablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTablet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTabletContextIdFromTablet: unsafe extern "system" fn(this: *mut *mut Self, pitablet: *mut ::core::ffi::c_void, ptcid: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTabletContextIdFromTablet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTabletFromTabletContextId: unsafe extern "system" fn(this: *mut *mut Self, tcid: u32, ppitablet: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTabletFromTabletContextId: usize,
    pub GetAllTabletContextIds: unsafe extern "system" fn(this: *mut *mut Self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStyluses: unsafe extern "system" fn(this: *mut *mut Self, ppiinkcursors: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStyluses: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStylusForId: unsafe extern "system" fn(this: *mut *mut Self, sid: u32, ppiinkcursor: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStylusForId: usize,
    pub SetDesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, cproperties: u32, ppropertyguids: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDesiredPacketDescription: unsafe extern "system" fn(this: *mut *mut Self, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetPacketDescriptionData: unsafe extern "system" fn(this: *mut *mut Self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRealTimeStylus {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2830851362, data2: 12612, data3: 19067, data4: [147, 205, 243, 74, 22, 190, 81, 58] };
}
#[repr(C)]
pub struct IRealTimeStylus2 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub FlicksEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FlicksEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFlicksEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFlicksEnabled: usize,
}
impl ::windows_sys::core::Interface for IRealTimeStylus2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3052578509, data2: 12665, data3: 19006, data4: [185, 196, 187, 88, 101, 150, 43, 226] };
}
#[repr(C)]
pub struct IRealTimeStylus3 {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub MultiTouchEnabled: unsafe extern "system" fn(this: *mut *mut Self, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MultiTouchEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultiTouchEnabled: unsafe extern "system" fn(this: *mut *mut Self, fenable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultiTouchEnabled: usize,
}
impl ::windows_sys::core::Interface for IRealTimeStylus3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3607244963, data2: 27014, data3: 16465, data4: [181, 122, 28, 246, 159, 77, 157, 181] };
}
#[repr(C)]
pub struct IRealTimeStylusSynchronization {
    pub base__: ::windows_sys::core::IUnknown,
    pub AcquireLock: unsafe extern "system" fn(this: *mut *mut Self, lock: RealTimeStylusLockType) -> ::windows_sys::core::HRESULT,
    pub ReleaseLock: unsafe extern "system" fn(this: *mut *mut Self, lock: RealTimeStylusLockType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRealTimeStylusSynchronization {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2861034168, data2: 43850, data3: 19690, data4: [181, 203, 70, 216, 76, 106, 37, 9] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISketchInk {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for ISketchInk {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3025548936, data2: 39147, data3: 17990, data4: [178, 121, 68, 218, 20, 212, 87, 72] };
}
#[repr(C)]
pub struct IStrokeBuilder {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStroke: unsafe extern "system" fn(this: *mut *mut Self, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStroke: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginStroke: unsafe extern "system" fn(this: *mut *mut Self, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginStroke: usize,
    pub AppendPackets: unsafe extern "system" fn(this: *mut *mut Self, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EndStroke: unsafe extern "system" fn(this: *mut *mut Self, tcid: u32, sid: u32, ppiinkstroke: *mut *mut ::core::ffi::c_void, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EndStroke: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Ink: unsafe extern "system" fn(this: *mut *mut Self, ppiinkobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Ink: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Ink: unsafe extern "system" fn(this: *mut *mut Self, piinkobj: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Ink: usize,
}
impl ::windows_sys::core::Interface for IStrokeBuilder {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2784841261, data2: 50251, data3: 16530, data4: [145, 119, 38, 9, 5, 235, 103, 43] };
}
#[repr(C)]
pub struct IStylusAsyncPlugin {
    pub base__: IStylusPlugin,
}
impl ::windows_sys::core::Interface for IStylusAsyncPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2815207514, data2: 12732, data3: 19666, data4: [170, 220, 50, 137, 163, 175, 17, 200] };
}
#[repr(C)]
pub struct IStylusPlugin {
    pub base__: ::windows_sys::core::IUnknown,
    pub RealTimeStylusEnabled: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> ::windows_sys::core::HRESULT,
    pub RealTimeStylusDisabled: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, ctcidcount: u32, ptcids: *const u32) -> ::windows_sys::core::HRESULT,
    pub StylusInRange: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32) -> ::windows_sys::core::HRESULT,
    pub StylusOutOfRange: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StylusDown: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StylusDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StylusUp: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StylusUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StylusButtonDown: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, sid: u32, pguidstylusbutton: *const ::windows_sys::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StylusButtonDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StylusButtonUp: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, sid: u32, pguidstylusbutton: *const ::windows_sys::core::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StylusButtonUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InAirPackets: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InAirPackets: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Packets: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Packets: usize,
    pub CustomStylusDataAdded: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pguidid: *const ::windows_sys::core::GUID, cbdata: u32, pbdata: *const u8) -> ::windows_sys::core::HRESULT,
    pub SystemEvent: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TabletAdded: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, pitablet: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TabletAdded: usize,
    pub TabletRemoved: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, itabletindex: i32) -> ::windows_sys::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void, piplugin: *mut ::core::ffi::c_void, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows_sys::core::HRESULT, lptrkey: *mut isize) -> ::windows_sys::core::HRESULT,
    pub UpdateMapping: unsafe extern "system" fn(this: *mut *mut Self, pirtssrc: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DataInterest: unsafe extern "system" fn(this: *mut *mut Self, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStylusPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2819897048, data2: 18263, data3: 20433, data4: [161, 133, 19, 63, 151, 198, 197, 69] };
}
#[repr(C)]
pub struct IStylusSyncPlugin {
    pub base__: IStylusPlugin,
}
impl ::windows_sys::core::Interface for IStylusSyncPlugin {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2706878836, data2: 18479, data3: 19825, data4: [163, 246, 58, 65, 221, 209, 27, 233] };
}
#[repr(C)]
pub struct ITextInputPanel {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AttachedEditWindow: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AttachedEditWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttachedEditWindow: unsafe extern "system" fn(this: *mut *mut Self, attachededitwindow: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttachedEditWindow: usize,
    pub CurrentInteractionMode: unsafe extern "system" fn(this: *mut *mut Self, currentinteractionmode: *mut InteractionMode) -> ::windows_sys::core::HRESULT,
    pub DefaultInPlaceState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut InPlaceState) -> ::windows_sys::core::HRESULT,
    pub SetDefaultInPlaceState: unsafe extern "system" fn(this: *mut *mut Self, state: InPlaceState) -> ::windows_sys::core::HRESULT,
    pub CurrentInPlaceState: unsafe extern "system" fn(this: *mut *mut Self, state: *mut InPlaceState) -> ::windows_sys::core::HRESULT,
    pub DefaultInputArea: unsafe extern "system" fn(this: *mut *mut Self, area: *mut PanelInputArea) -> ::windows_sys::core::HRESULT,
    pub SetDefaultInputArea: unsafe extern "system" fn(this: *mut *mut Self, area: PanelInputArea) -> ::windows_sys::core::HRESULT,
    pub CurrentInputArea: unsafe extern "system" fn(this: *mut *mut Self, area: *mut PanelInputArea) -> ::windows_sys::core::HRESULT,
    pub CurrentCorrectionMode: unsafe extern "system" fn(this: *mut *mut Self, mode: *mut CorrectionMode) -> ::windows_sys::core::HRESULT,
    pub PreferredInPlaceDirection: unsafe extern "system" fn(this: *mut *mut Self, direction: *mut InPlaceDirection) -> ::windows_sys::core::HRESULT,
    pub SetPreferredInPlaceDirection: unsafe extern "system" fn(this: *mut *mut Self, direction: InPlaceDirection) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandPostInsertionCorrection: unsafe extern "system" fn(this: *mut *mut Self, expand: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandPostInsertionCorrection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExpandPostInsertionCorrection: unsafe extern "system" fn(this: *mut *mut Self, expand: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExpandPostInsertionCorrection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceVisibleOnFocus: unsafe extern "system" fn(this: *mut *mut Self, visible: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceVisibleOnFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInPlaceVisibleOnFocus: unsafe extern "system" fn(this: *mut *mut Self, visible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInPlaceVisibleOnFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceBoundingRectangle: unsafe extern "system" fn(this: *mut *mut Self, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceBoundingRectangle: usize,
    pub PopUpCorrectionHeight: unsafe extern "system" fn(this: *mut *mut Self, height: *mut i32) -> ::windows_sys::core::HRESULT,
    pub PopDownCorrectionHeight: unsafe extern "system" fn(this: *mut *mut Self, height: *mut i32) -> ::windows_sys::core::HRESULT,
    pub CommitPendingInput: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInPlaceVisibility: unsafe extern "system" fn(this: *mut *mut Self, visible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInPlaceVisibility: usize,
    pub SetInPlacePosition: unsafe extern "system" fn(this: *mut *mut Self, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows_sys::core::HRESULT,
    pub SetInPlaceHoverTargetPosition: unsafe extern "system" fn(this: *mut *mut Self, xposition: i32, yposition: i32) -> ::windows_sys::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut *mut Self, eventsink: *mut ::core::ffi::c_void, eventmask: u32) -> ::windows_sys::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut *mut Self, eventsink: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITextInputPanel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1802134949, data2: 27379, data3: 18114, data4: [182, 234, 86, 205, 31, 128, 223, 113] };
}
#[repr(C)]
pub struct ITextInputPanelEventSink {
    pub base__: ::windows_sys::core::IUnknown,
    pub InPlaceStateChanging: unsafe extern "system" fn(this: *mut *mut Self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_sys::core::HRESULT,
    pub InPlaceStateChanged: unsafe extern "system" fn(this: *mut *mut Self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceSizeChanging: unsafe extern "system" fn(this: *mut *mut Self, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceSizeChanging: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceSizeChanged: unsafe extern "system" fn(this: *mut *mut Self, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceSizeChanged: usize,
    pub InputAreaChanging: unsafe extern "system" fn(this: *mut *mut Self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_sys::core::HRESULT,
    pub InputAreaChanged: unsafe extern "system" fn(this: *mut *mut Self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows_sys::core::HRESULT,
    pub CorrectionModeChanging: unsafe extern "system" fn(this: *mut *mut Self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_sys::core::HRESULT,
    pub CorrectionModeChanged: unsafe extern "system" fn(this: *mut *mut Self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceVisibilityChanging: unsafe extern "system" fn(this: *mut *mut Self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceVisibilityChanging: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InPlaceVisibilityChanged: unsafe extern "system" fn(this: *mut *mut Self, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InPlaceVisibilityChanged: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TextInserting: unsafe extern "system" fn(this: *mut *mut Self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TextInserting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TextInserted: unsafe extern "system" fn(this: *mut *mut Self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TextInserted: usize,
}
impl ::windows_sys::core::Interface for ITextInputPanelEventSink {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 659948552, data2: 36452, data3: 20449, data4: [128, 78, 66, 18, 1, 88, 75, 49] };
}
#[repr(C)]
pub struct ITextInputPanelRunInfo {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTipRunning: unsafe extern "system" fn(this: *mut *mut Self, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTipRunning: usize,
}
impl ::windows_sys::core::Interface for ITextInputPanelRunInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2671920488, data2: 6432, data3: 18636, data4: [152, 17, 169, 147, 203, 245, 173, 186] };
}
#[repr(C)]
pub struct ITipAutoCompleteClient {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseProvider: unsafe extern "system" fn(this: *mut *mut Self, hwndfield: super::super::Foundation::HWND, piprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnadviseProvider: unsafe extern "system" fn(this: *mut *mut Self, hwndfield: super::super::Foundation::HWND, piprovider: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnadviseProvider: usize,
    pub UserSelection: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PreferredRects: unsafe extern "system" fn(this: *mut *mut Self, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreferredRects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestShowUI: unsafe extern "system" fn(this: *mut *mut Self, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestShowUI: usize,
}
impl ::windows_sys::core::Interface for ITipAutoCompleteClient {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1577553411, data2: 33381, data3: 19390, data4: [148, 135, 210, 66, 237, 190, 249, 16] };
}
#[repr(C)]
pub struct ITipAutoCompleteProvider {
    pub base__: ::windows_sys::core::IUnknown,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdatePendingText: unsafe extern "system" fn(this: *mut *mut Self, bstrpendingtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdatePendingText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut *mut Self, fshow: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
impl ::windows_sys::core::Interface for ITipAutoCompleteProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2087515245, data2: 33796, data3: 18105, data4: [173, 51, 245, 182, 3, 109, 64, 7] };
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InPlaceDirection = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceDirection_Auto: InPlaceDirection = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceDirection_Bottom: InPlaceDirection = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceDirection_Top: InPlaceDirection = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InPlaceState = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceState_Auto: InPlaceState = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceState_HoverTarget: InPlaceState = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlaceState_Expanded: InPlaceState = 2i32;
pub const Ink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 333335106, data2: 36129, data3: 19598, data4: [191, 156, 143, 105, 203, 6, 143, 202] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkApplicationGesture = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_AllGestures: InkApplicationGesture = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_NoGesture: InkApplicationGesture = 61440i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Scratchout: InkApplicationGesture = 61441i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Triangle: InkApplicationGesture = 61442i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Square: InkApplicationGesture = 61443i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Star: InkApplicationGesture = 61444i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Check: InkApplicationGesture = 61445i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Curlicue: InkApplicationGesture = 61456i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DoubleCurlicue: InkApplicationGesture = 61457i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Circle: InkApplicationGesture = 61472i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DoubleCircle: InkApplicationGesture = 61473i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_SemiCircleLeft: InkApplicationGesture = 61480i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_SemiCircleRight: InkApplicationGesture = 61481i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ChevronUp: InkApplicationGesture = 61488i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ChevronDown: InkApplicationGesture = 61489i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ChevronLeft: InkApplicationGesture = 61490i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ChevronRight: InkApplicationGesture = 61491i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ArrowUp: InkApplicationGesture = 61496i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ArrowDown: InkApplicationGesture = 61497i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ArrowLeft: InkApplicationGesture = 61498i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_ArrowRight: InkApplicationGesture = 61499i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Up: InkApplicationGesture = 61528i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Down: InkApplicationGesture = 61529i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Left: InkApplicationGesture = 61530i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Right: InkApplicationGesture = 61531i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_UpDown: InkApplicationGesture = 61536i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DownUp: InkApplicationGesture = 61537i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_LeftRight: InkApplicationGesture = 61538i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_RightLeft: InkApplicationGesture = 61539i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_UpLeftLong: InkApplicationGesture = 61540i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_UpRightLong: InkApplicationGesture = 61541i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DownLeftLong: InkApplicationGesture = 61542i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DownRightLong: InkApplicationGesture = 61543i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_UpLeft: InkApplicationGesture = 61544i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_UpRight: InkApplicationGesture = 61545i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DownLeft: InkApplicationGesture = 61546i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DownRight: InkApplicationGesture = 61547i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_LeftUp: InkApplicationGesture = 61548i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_LeftDown: InkApplicationGesture = 61549i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_RightUp: InkApplicationGesture = 61550i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_RightDown: InkApplicationGesture = 61551i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Exclamation: InkApplicationGesture = 61604i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_Tap: InkApplicationGesture = 61680i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IAG_DoubleTap: InkApplicationGesture = 61681i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkBoundingBoxMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IBBM_Default: InkBoundingBoxMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IBBM_NoCurveFit: InkBoundingBoxMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IBBM_CurveFit: InkBoundingBoxMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IBBM_PointsOnly: InkBoundingBoxMode = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IBBM_Union: InkBoundingBoxMode = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkClipboardFormats = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_None: InkClipboardFormats = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_InkSerializedFormat: InkClipboardFormats = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_SketchInk: InkClipboardFormats = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_TextInk: InkClipboardFormats = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_EnhancedMetafile: InkClipboardFormats = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_Metafile: InkClipboardFormats = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_Bitmap: InkClipboardFormats = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_PasteMask: InkClipboardFormats = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_CopyMask: InkClipboardFormats = 127i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICF_Default: InkClipboardFormats = 127i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkClipboardModes = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICB_Copy: InkClipboardModes = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICB_Cut: InkClipboardModes = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICB_ExtractOnly: InkClipboardModes = 48i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICB_DelayedCopy: InkClipboardModes = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICB_Default: InkClipboardModes = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkCollectionMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICM_InkOnly: InkCollectionMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICM_GestureOnly: InkCollectionMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICM_InkAndGesture: InkCollectionMode = 2i32;
pub const InkCollector: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1140528467, data2: 44404, data3: 20200, data4: [136, 228, 62, 109, 170, 201, 21, 219] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InkCollectorClipInkToMargin: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkCollectorEventInterest = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_DefaultEvents: InkCollectorEventInterest = -1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_CursorDown: InkCollectorEventInterest = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_Stroke: InkCollectorEventInterest = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_NewPackets: InkCollectorEventInterest = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_NewInAirPackets: InkCollectorEventInterest = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_CursorButtonDown: InkCollectorEventInterest = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_CursorButtonUp: InkCollectorEventInterest = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_CursorInRange: InkCollectorEventInterest = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_CursorOutOfRange: InkCollectorEventInterest = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_SystemGesture: InkCollectorEventInterest = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_TabletAdded: InkCollectorEventInterest = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_TabletRemoved: InkCollectorEventInterest = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_MouseDown: InkCollectorEventInterest = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_MouseMove: InkCollectorEventInterest = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_MouseUp: InkCollectorEventInterest = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_MouseWheel: InkCollectorEventInterest = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_DblClick: InkCollectorEventInterest = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICEI_AllEvents: InkCollectorEventInterest = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkCursorButtonState = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICBS_Unavailable: InkCursorButtonState = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICBS_Up: InkCursorButtonState = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ICBS_Down: InkCursorButtonState = 2i32;
pub const InkDisp: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2474383924, data2: 5405, data3: 17936, data4: [156, 166, 168, 204, 155, 219, 93, 131] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkDisplayMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDM_Ink: InkDisplayMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDM_Text: InkDisplayMode = 1i32;
pub const InkDivider: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2287269536, data2: 18051, data3: 19175, data4: [145, 145, 117, 47, 230, 70, 18, 195] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkDivisionType = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDT_Segment: InkDivisionType = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDT_Line: InkDivisionType = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDT_Paragraph: InkDivisionType = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IDT_Drawing: InkDivisionType = 3i32;
pub const InkDrawingAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3636408994, data2: 1445, data3: 17603, data4: [179, 170, 94, 128, 172, 125, 37, 118] };
pub const InkEdit: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3855243765, data2: 22468, data3: 19928, data4: [155, 214, 29, 238, 237, 210, 122, 244] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkEditStatus = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IES_Idle: InkEditStatus = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IES_Collecting: InkEditStatus = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IES_Recognizing: InkEditStatus = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkExtractFlags = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEF_CopyFromOriginal: InkExtractFlags = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEF_RemoveFromOriginal: InkExtractFlags = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEF_Default: InkExtractFlags = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkInsertMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEM_InsertText: InkInsertMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEM_InsertInk: InkInsertMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InkMaxTransparencyValue: i32 = 255i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InkMinTransparencyValue: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEM_Disabled: InkMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEM_Ink: InkMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IEM_InkAndGesture: InkMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkMouseButton = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_Left: InkMouseButton = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_Right: InkMouseButton = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_Middle: InkMouseButton = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkMousePointer = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Default: InkMousePointer = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Arrow: InkMousePointer = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Crosshair: InkMousePointer = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Ibeam: InkMousePointer = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_SizeNESW: InkMousePointer = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_SizeNS: InkMousePointer = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_SizeNWSE: InkMousePointer = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_SizeWE: InkMousePointer = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_UpArrow: InkMousePointer = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Hourglass: InkMousePointer = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_NoDrop: InkMousePointer = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_ArrowHourglass: InkMousePointer = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_ArrowQuestion: InkMousePointer = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_SizeAll: InkMousePointer = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Hand: InkMousePointer = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMP_Custom: InkMousePointer = 99i32;
pub const InkOverlay: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1708131910, data2: 52707, data3: 19080, data4: [145, 99, 103, 105, 240, 241, 169, 125] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkOverlayAttachMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOAM_Behind: InkOverlayAttachMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOAM_InFront: InkOverlayAttachMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkOverlayEditingMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOEM_Ink: InkOverlayEditingMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOEM_Delete: InkOverlayEditingMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOEM_Select: InkOverlayEditingMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkOverlayEraserMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOERM_StrokeErase: InkOverlayEraserMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IOERM_PointErase: InkOverlayEraserMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkPenTip = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPT_Ball: InkPenTip = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPT_Rectangle: InkPenTip = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkPersistenceCompressionMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPCM_Default: InkPersistenceCompressionMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPCM_MaximumCompression: InkPersistenceCompressionMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPCM_NoCompression: InkPersistenceCompressionMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkPersistenceFormat = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPF_InkSerializedFormat: InkPersistenceFormat = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPF_Base64InkSerializedFormat: InkPersistenceFormat = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPF_GIF: InkPersistenceFormat = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPF_Base64GIF: InkPersistenceFormat = 3i32;
pub const InkPicture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 77718867, data2: 65078, data3: 20446, data4: [134, 94, 52, 65, 148, 230, 148, 36] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkPictureSizeMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPSM_AutoSize: InkPictureSizeMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPSM_CenterImage: InkPictureSizeMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPSM_Normal: InkPictureSizeMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IPSM_StretchImage: InkPictureSizeMode = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRasterOperation = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_Black: InkRasterOperation = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_NotMergePen: InkRasterOperation = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MaskNotPen: InkRasterOperation = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_NotCopyPen: InkRasterOperation = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MaskPenNot: InkRasterOperation = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_Not: InkRasterOperation = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_XOrPen: InkRasterOperation = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_NotMaskPen: InkRasterOperation = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MaskPen: InkRasterOperation = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_NotXOrPen: InkRasterOperation = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_NoOperation: InkRasterOperation = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MergeNotPen: InkRasterOperation = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_CopyPen: InkRasterOperation = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MergePenNot: InkRasterOperation = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_MergePen: InkRasterOperation = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRO_White: InkRasterOperation = 16i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct InkRecoGuide {
    pub rectWritingBox: super::super::Foundation::RECT,
    pub rectDrawnBox: super::super::Foundation::RECT,
    pub cRows: i32,
    pub cColumns: i32,
    pub midline: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for InkRecoGuide {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for InkRecoGuide {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognitionAlternatesSelection = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRAS_Start: InkRecognitionAlternatesSelection = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRAS_DefaultCount: InkRecognitionAlternatesSelection = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRAS_All: InkRecognitionAlternatesSelection = -1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognitionConfidence = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Strong: InkRecognitionConfidence = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Intermediate: InkRecognitionConfidence = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Poor: InkRecognitionConfidence = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognitionModes = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_None: InkRecognitionModes = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_WordModeOnly: InkRecognitionModes = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_Coerce: InkRecognitionModes = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_TopInkBreaksOnly: InkRecognitionModes = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_PrefixOk: InkRecognitionModes = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_LineMode: InkRecognitionModes = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_DisablePersonalization: InkRecognitionModes = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_AutoSpace: InkRecognitionModes = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRM_Max: InkRecognitionModes = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognitionStatus = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_NoError: InkRecognitionStatus = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_Interrupted: InkRecognitionStatus = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_ProcessFailed: InkRecognitionStatus = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_InkAddedFailed: InkRecognitionStatus = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetAutoCompletionModeFailed: InkRecognitionStatus = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetStrokesFailed: InkRecognitionStatus = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetGuideFailed: InkRecognitionStatus = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetFlagsFailed: InkRecognitionStatus = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetFactoidFailed: InkRecognitionStatus = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetPrefixSuffixFailed: InkRecognitionStatus = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRS_SetWordListFailed: InkRecognitionStatus = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognizerCapabilities = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_DontCare: InkRecognizerCapabilities = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Object: InkRecognizerCapabilities = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_FreeInput: InkRecognizerCapabilities = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_LinedInput: InkRecognizerCapabilities = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_BoxedInput: InkRecognizerCapabilities = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_CharacterAutoCompletionInput: InkRecognizerCapabilities = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_RightAndDown: InkRecognizerCapabilities = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_LeftAndDown: InkRecognizerCapabilities = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_DownAndLeft: InkRecognizerCapabilities = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_DownAndRight: InkRecognizerCapabilities = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_ArbitraryAngle: InkRecognizerCapabilities = 1024i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Lattice: InkRecognizerCapabilities = 2048i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_AdviseInkChange: InkRecognizerCapabilities = 4096i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_StrokeReorder: InkRecognizerCapabilities = 8192i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Personalizable: InkRecognizerCapabilities = 16384i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_PrefersArbitraryAngle: InkRecognizerCapabilities = 32768i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_PrefersParagraphBreaking: InkRecognizerCapabilities = 65536i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_PrefersSegmentation: InkRecognizerCapabilities = 131072i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Cursive: InkRecognizerCapabilities = 262144i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_TextPrediction: InkRecognizerCapabilities = 524288i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Alpha: InkRecognizerCapabilities = 1048576i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRC_Beta: InkRecognizerCapabilities = 2097152i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkRecognizerCharacterAutoCompletionMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRCACM_Full: InkRecognizerCharacterAutoCompletionMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRCACM_Prefix: InkRecognizerCharacterAutoCompletionMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IRCACM_Random: InkRecognizerCharacterAutoCompletionMode = 2i32;
pub const InkRecognizerContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2864998967, data2: 37417, data3: 20416, data4: [140, 206, 68, 151, 86, 155, 244, 209] };
pub const InkRecognizerGuide: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2272319809, data2: 42554, data3: 18033, data4: [163, 117, 40, 85, 161, 142, 186, 115] };
pub const InkRecognizers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2681530376, data2: 63206, data3: 20069, data4: [152, 211, 170, 57, 5, 76, 18, 85] };
pub const InkRectangle: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1135637286, data2: 43744, data3: 19298, data4: [168, 61, 95, 215, 104, 183, 53, 60] };
pub const InkRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2619131620, data2: 55275, data3: 20203, data4: [144, 145, 21, 167, 200, 121, 30, 217] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkSelectionConstants = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISC_FirstElement: InkSelectionConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISC_AllElements: InkSelectionConstants = -1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkShiftKeyModifierFlags = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IKM_Shift: InkShiftKeyModifierFlags = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IKM_Control: InkShiftKeyModifierFlags = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IKM_Alt: InkShiftKeyModifierFlags = 4i32;
pub const InkStrokes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223987644, data2: 9230, data3: 18528, data4: [176, 121, 161, 233, 77, 61, 44, 134] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InkSystemGesture = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_Tap: InkSystemGesture = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_DoubleTap: InkSystemGesture = 17i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_RightTap: InkSystemGesture = 18i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_Drag: InkSystemGesture = 19i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_RightDrag: InkSystemGesture = 20i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_HoldEnter: InkSystemGesture = 21i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_HoldLeave: InkSystemGesture = 22i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_HoverEnter: InkSystemGesture = 23i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_HoverLeave: InkSystemGesture = 24i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const ISG_Flick: InkSystemGesture = 31i32;
pub const InkTablets: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1850723090, data2: 20746, data3: 19776, data4: [147, 4, 29, 161, 10, 233, 20, 124] };
pub const InkTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822442812, data2: 5731, data3: 19064, data4: [161, 167, 34, 55, 93, 254, 186, 238] };
pub const InkWordList: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2649247892, data2: 63263, data3: 17649, data4: [132, 113, 21, 162, 250, 118, 252, 243] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type InteractionMode = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InteractionMode_InPlace: InteractionMode = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InteractionMode_Floating: InteractionMode = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InteractionMode_DockedTop: InteractionMode = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InteractionMode_DockedBottom: InteractionMode = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type KEYMODIFIER = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_CONTROL: KEYMODIFIER = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_MENU: KEYMODIFIER = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_SHIFT: KEYMODIFIER = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_WIN: KEYMODIFIER = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_ALTGR: KEYMODIFIER = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const KEYMODIFIER_EXT: KEYMODIFIER = 32i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LATTICE_METRICS {
    pub lsBaseline: LINE_SEGMENT,
    pub iMidlineOffset: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LATTICE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LATTICE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type LINE_METRICS = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const LM_BASELINE: LINE_METRICS = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const LM_MIDLINE: LINE_METRICS = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const LM_ASCENDER: LINE_METRICS = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const LM_DESCENDER: LINE_METRICS = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LINE_SEGMENT {
    pub PtA: super::super::Foundation::POINT,
    pub PtB: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINE_SEGMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINE_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MAX_FRIENDLYNAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MAX_LANGUAGES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MAX_VENDORNAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICROSOFT_PENINPUT_PANEL_PROPERTY_T: &str = "Microsoft PenInputPanel 1.5";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: &str = "Microsoft TIP ComboBox List Window Identifier";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: &str = "Microsoft TIP No Insert Option";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICROSOFT_TIP_OPENING_MSG: &str = "TabletInputPanelOpening";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: &str = "Microsoft TIP URL Experience";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type MICUIELEMENT = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_WRITE: MICUIELEMENT = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_ERASE: MICUIELEMENT = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_CORRECT: MICUIELEMENT = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_CLEAR: MICUIELEMENT = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_UNDO: MICUIELEMENT = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_REDO: MICUIELEMENT = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_INSERT: MICUIELEMENT = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_BUTTON_CANCEL: MICUIELEMENT = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_INKPANEL_BACKGROUND: MICUIELEMENT = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENT_RESULTPANEL_BACKGROUND: MICUIELEMENT = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type MICUIELEMENTSTATE = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENTSTATE_NORMAL: MICUIELEMENTSTATE = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENTSTATE_HOT: MICUIELEMENTSTATE = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENTSTATE_PRESSED: MICUIELEMENTSTATE = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MICUIELEMENTSTATE_DISABLED: MICUIELEMENTSTATE = 4i32;
pub const MathInputControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3311501676, data2: 5336, data3: 16528, data4: [131, 12, 152, 217, 148, 178, 28, 123] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type MouseButton = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const NO_BUTTON: MouseButton = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const LEFT_BUTTON: MouseButton = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RIGHT_BUTTON: MouseButton = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const MIDDLE_BUTTON: MouseButton = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const NUM_FLICK_DIRECTIONS: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PACKET_DESCRIPTION {}
impl ::core::clone::Clone for PACKET_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct PACKET_PROPERTY {
    pub guid: ::windows_sys::core::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
impl ::core::marker::Copy for PACKET_PROPERTY {}
impl ::core::clone::Clone for PACKET_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
impl ::core::marker::Copy for PROPERTY_METRICS {}
impl ::core::clone::Clone for PROPERTY_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type PROPERTY_UNITS = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_DEFAULT: PROPERTY_UNITS = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_INCHES: PROPERTY_UNITS = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_CENTIMETERS: PROPERTY_UNITS = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_DEGREES: PROPERTY_UNITS = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_RADIANS: PROPERTY_UNITS = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_SECONDS: PROPERTY_UNITS = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_POUNDS: PROPERTY_UNITS = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_GRAMS: PROPERTY_UNITS = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_SILINEAR: PROPERTY_UNITS = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_SIROTATION: PROPERTY_UNITS = 9i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_ENGLINEAR: PROPERTY_UNITS = 10i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_ENGROTATION: PROPERTY_UNITS = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_SLUGS: PROPERTY_UNITS = 12i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_KELVIN: PROPERTY_UNITS = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_FAHRENHEIT: PROPERTY_UNITS = 14i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_AMPERE: PROPERTY_UNITS = 15i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PROPERTY_UNITS_CANDELA: PROPERTY_UNITS = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type PanelInputArea = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PanelInputArea_Auto: PanelInputArea = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PanelInputArea_Keyboard: PanelInputArea = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PanelInputArea_WritingPad: PanelInputArea = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PanelInputArea_CharacterPad: PanelInputArea = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type PanelType = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PT_Default: PanelType = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PT_Inactive: PanelType = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PT_Handwriting: PanelType = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const PT_Keyboard: PanelType = 3i32;
pub const PenInputPanel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4148487318, data2: 7002, data3: 18590, data4: [129, 220, 251, 215, 172, 98, 152, 168] };
pub const PenInputPanel_Internal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2150309817, data2: 1387, data3: 18208, data4: [176, 204, 128, 210, 59, 113, 23, 30] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type PfnRecoCallback = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut u8, param2: HRECOCONTEXT) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOCONF_HIGHCONFIDENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOCONF_LOWCONFIDENCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOCONF_MEDIUMCONFIDENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOCONF_NOTSET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_AUTOSPACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_COERCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_DISABLEPERSONALIZATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_LINEMODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_PREFIXOK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_SINGLESEG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECOFLAG_WORDMODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_ATTRS {
    pub dwRecoCapabilityFlags: u32,
    pub awcVendorName: [u16; 32],
    pub awcFriendlyName: [u16; 64],
    pub awLanguageId: [u16; 64],
}
impl ::core::marker::Copy for RECO_ATTRS {}
impl ::core::clone::Clone for RECO_ATTRS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_GUIDE {
    pub xOrigin: i32,
    pub yOrigin: i32,
    pub cxBox: i32,
    pub cyBox: i32,
    pub cxBase: i32,
    pub cyBase: i32,
    pub cHorzBox: i32,
    pub cVertBox: i32,
    pub cyMid: i32,
}
impl ::core::marker::Copy for RECO_GUIDE {}
impl ::core::clone::Clone for RECO_GUIDE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_LATTICE {
    pub ulColumnCount: u32,
    pub pLatticeColumns: *mut RECO_LATTICE_COLUMN,
    pub ulPropertyCount: u32,
    pub pGuidProperties: *mut ::windows_sys::core::GUID,
    pub ulBestResultColumnCount: u32,
    pub pulBestResultColumns: *mut u32,
    pub pulBestResultIndexes: *mut u32,
}
impl ::core::marker::Copy for RECO_LATTICE {}
impl ::core::clone::Clone for RECO_LATTICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_LATTICE_COLUMN {
    pub key: u32,
    pub cpProp: RECO_LATTICE_PROPERTIES,
    pub cStrokes: u32,
    pub pStrokes: *mut u32,
    pub cLatticeElements: u32,
    pub pLatticeElements: *mut RECO_LATTICE_ELEMENT,
}
impl ::core::marker::Copy for RECO_LATTICE_COLUMN {}
impl ::core::clone::Clone for RECO_LATTICE_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_LATTICE_ELEMENT {
    pub score: i32,
    pub r#type: u16,
    pub pData: *mut u8,
    pub ulNextColumn: u32,
    pub ulStrokeNumber: u32,
    pub epProp: RECO_LATTICE_PROPERTIES,
}
impl ::core::marker::Copy for RECO_LATTICE_ELEMENT {}
impl ::core::clone::Clone for RECO_LATTICE_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_LATTICE_PROPERTIES {
    pub cProperties: u32,
    pub apProps: *mut *mut RECO_LATTICE_PROPERTY,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTIES {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_LATTICE_PROPERTY {
    pub guidProperty: ::windows_sys::core::GUID,
    pub cbPropertyValue: u16,
    pub pPropertyValue: *mut u8,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTY {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct RECO_RANGE {
    pub iwcBegin: u32,
    pub cCount: u32,
}
impl ::core::marker::Copy for RECO_RANGE {}
impl ::core::clone::Clone for RECO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_ADVISEINKCHANGE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_ARBITRARY_ANGLE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_BOXED_INPUT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_CAC_INPUT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_DONTCARE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_DOWN_AND_LEFT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_DOWN_AND_RIGHT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_FREE_INPUT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_LATTICE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_LEFT_AND_DOWN: i32 = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_LINED_INPUT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_OBJECT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_PERFORMSLINEBREAKING: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_PERSONALIZABLE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_REQUIRESSEGMENTATIONBREAKING: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_RIGHT_AND_DOWN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RF_STROKEREORDER: i32 = 8192i32;
pub const RealTimeStylus: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3798677101, data2: 63896, data3: 17358, data4: [131, 111, 203, 109, 144, 68, 50, 176] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type RealTimeStylusDataInterest = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_AllData: RealTimeStylusDataInterest = -1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_None: RealTimeStylusDataInterest = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_Error: RealTimeStylusDataInterest = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_RealTimeStylusEnabled: RealTimeStylusDataInterest = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_RealTimeStylusDisabled: RealTimeStylusDataInterest = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusNew: RealTimeStylusDataInterest = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusInRange: RealTimeStylusDataInterest = 16i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_InAirPackets: RealTimeStylusDataInterest = 32i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusOutOfRange: RealTimeStylusDataInterest = 64i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusDown: RealTimeStylusDataInterest = 128i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_Packets: RealTimeStylusDataInterest = 256i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusUp: RealTimeStylusDataInterest = 512i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusButtonUp: RealTimeStylusDataInterest = 1024i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_StylusButtonDown: RealTimeStylusDataInterest = 2048i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_SystemEvents: RealTimeStylusDataInterest = 4096i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_TabletAdded: RealTimeStylusDataInterest = 8192i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_TabletRemoved: RealTimeStylusDataInterest = 16384i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_CustomStylusDataAdded: RealTimeStylusDataInterest = 32768i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_UpdateMapping: RealTimeStylusDataInterest = 65536i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSDI_DefaultEvents: RealTimeStylusDataInterest = 37766i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type RealTimeStylusLockType = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_ObjLock: RealTimeStylusLockType = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_SyncEventLock: RealTimeStylusLockType = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_AsyncEventLock: RealTimeStylusLockType = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_ExcludeCallback: RealTimeStylusLockType = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_SyncObjLock: RealTimeStylusLockType = 11i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RTSLT_AsyncObjLock: RealTimeStylusLockType = 13i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SAFE_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type SCROLLDIRECTION = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SCROLLDIRECTION_UP: SCROLLDIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SCROLLDIRECTION_DOWN: SCROLLDIRECTION = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
impl ::core::marker::Copy for STROKE_RANGE {}
impl ::core::clone::Clone for STROKE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_ALTITUDEORIENTATION: &str = "{82DEC5C7-F6BA-4906-894F-66D68DFC456C}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_AZIMUTHORIENTATION: &str = "{029123B4-8828-410B-B250-A0536595E5DC}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_BUTTONPRESSURE: &str = "{8B7FEFC4-96AA-4BFE-AC26-8A5F0BE07BF5}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_DEVICE_CONTACT_ID: &str = "{02585B91-049B-4750-9615-DF8948AB3C9C}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_FINGERCONTACTCONFIDENCE: &str = "{E706C804-57F0-4F00-8A0C-853D57789BE9}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_HEIGHT: &str = "{E61858D2-E447-4218-9D3F-18865C203DF4}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_NORMALPRESSURE: &str = "{7307502D-F9F4-4E18-B3F2-2CE1B1A3610C}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_PAKETSTATUS: &str = "{6E0E07BF-AFE7-4CF7-87D1-AF6446208418}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_PITCHROTATION: &str = "{7F7E57B7-BE37-4BE1-A356-7A84160E1893}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_ROLLROTATION: &str = "{5D5D5E56-6BA9-4C5B-9FB0-851C91714E56}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_SERIALNUMBER: &str = "{78A81B56-0935-4493-BAAE-00541A8A16C4}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_TANGENTPRESSURE: &str = "{6DA4488B-5244-41EC-905B-32D89AB80809}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_TIMERTICK: &str = "{436510C5-FED3-45D1-8B76-71D3EA7A829D}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_TWISTORIENTATION: &str = "{0D324960-13B2-41E4-ACE6-7AE9D43D2D3B}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_WIDTH: &str = "{BAABE94D-2712-48F5-BE9D-8F8B5EA0711A}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_X: &str = "{598A6A8F-52C0-4BA0-93AF-AF357411A561}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_XTILTORIENTATION: &str = "{A8D07B3A-8BF0-40B0-95A9-B80A6BB787BF}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_Y: &str = "{B53F9F75-04E0-4498-A7EE-C30DBB5A9011}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_YAWROTATION: &str = "{6A849980-7C3A-45B7-AA82-90A262950E89}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_YTILTORIENTATION: &str = "{0E932389-1D77-43AF-AC00-5B950D6D4B2D}";
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const STR_GUID_Z: &str = "{735ADB30-0EBB-4788-A0E4-0F316490055D}";
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
impl ::core::marker::Copy for SYSTEM_EVENT_DATA {}
impl ::core::clone::Clone for SYSTEM_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type ScrollBarsConstants = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfNone: ScrollBarsConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfHorizontal: ScrollBarsConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfVertical: ScrollBarsConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfBoth: ScrollBarsConstants = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type SelAlignmentConstants = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfLeft: SelAlignmentConstants = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfRight: SelAlignmentConstants = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const rtfCenter: SelAlignmentConstants = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type SelectionHitResult = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_None: SelectionHitResult = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_NW: SelectionHitResult = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_SE: SelectionHitResult = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_NE: SelectionHitResult = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_SW: SelectionHitResult = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_E: SelectionHitResult = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_W: SelectionHitResult = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_N: SelectionHitResult = 7i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_S: SelectionHitResult = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SHR_Selection: SelectionHitResult = 9i32;
pub const SketchInk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4029223041, data2: 59516, data3: 19975, data4: [151, 218, 160, 160, 55, 97, 229, 134] };
pub const StrokeBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3893415655, data2: 28241, data3: 19632, data4: [170, 58, 11, 152, 91, 112, 218, 247] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct StylusInfo {
    pub tcid: u32,
    pub cid: u32,
    pub bIsInvertedCursor: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for StylusInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for StylusInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type StylusQueue = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const SyncStylusQueue: StylusQueue = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const AsyncStylusQueueImmediate: StylusQueue = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const AsyncStylusQueue: StylusQueue = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_FLICKS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type TabletDeviceKind = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TDK_Mouse: TabletDeviceKind = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TDK_Pen: TabletDeviceKind = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TDK_Touch: TabletDeviceKind = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type TabletHardwareCapabilities = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const THWC_Integrated: TabletHardwareCapabilities = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const THWC_CursorMustTouch: TabletHardwareCapabilities = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const THWC_HardProximity: TabletHardwareCapabilities = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const THWC_CursorsHavePhysicalIds: TabletHardwareCapabilities = 8i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type TabletPropertyMetricUnit = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Default: TabletPropertyMetricUnit = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Inches: TabletPropertyMetricUnit = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Centimeters: TabletPropertyMetricUnit = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Degrees: TabletPropertyMetricUnit = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Radians: TabletPropertyMetricUnit = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Seconds: TabletPropertyMetricUnit = 5i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Pounds: TabletPropertyMetricUnit = 6i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TPMU_Grams: TabletPropertyMetricUnit = 7i32;
pub const TextInputPanel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4189161943, data2: 8843, data3: 20267, data4: [134, 80, 185, 127, 89, 224, 44, 140] };
pub const TipAutoCompleteClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2155617900, data2: 7424, data3: 17727, data4: [185, 32, 182, 27, 183, 205, 217, 151] };
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type VisualState = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const InPlace: VisualState = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const Floating: VisualState = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DockedTop: VisualState = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const DockedBottom: VisualState = 3i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const Closed: VisualState = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_ADDED: u32 = 712u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_DEFBASE: u32 = 704u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_DELETED: u32 = 713u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_FLICK: u32 = 715u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_MAXOFFSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716u32;
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkCollectorEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkCollectorEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 296059890, data2: 28973, data3: 20458, data4: [171, 207, 171, 74, 243, 142, 160, 107] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkEditEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkEditEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3820009367, data2: 42798, data3: 18139, data4: [160, 215, 108, 158, 186, 142, 155, 188] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1115363429, data2: 51775, data3: 18330, data4: [131, 169, 15, 66, 15, 42, 0, 115] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkOverlayEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkOverlayEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 823630697, data2: 58723, data3: 18590, data4: [177, 111, 113, 47, 30, 138, 6, 81] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkPictureEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkPictureEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1627344878, data2: 8959, data3: 17540, data4: [172, 193, 211, 8, 217, 205, 126, 163] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkRecognitionEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkRecognitionEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 398256431, data2: 11809, data3: 18429, data4: [157, 51, 60, 106, 251, 253, 140, 89] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IInkStrokesEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IInkStrokesEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4080030700, data2: 23845, data3: 17162, data4: [146, 143, 118, 166, 73, 29, 222, 21] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IMathInputControlEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IMathInputControlEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1748186805, data2: 42109, data3: 17240, data4: [150, 249, 135, 90, 71, 42, 231, 10] };
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IPenInputPanelEvents {
    pub base__: super::super::System::Com::IDispatch,
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_sys::core::Interface for _IPenInputPanelEvents {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3085208026, data2: 14105, data3: 17311, data4: [132, 143, 231, 172, 189, 130, 15, 23] };
}
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type enumGetCandidateFlags = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TCF_ALLOW_RECOGNITION: enumGetCandidateFlags = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const TCF_FORCE_RECOGNITION: enumGetCandidateFlags = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type enumINKMETRIC_FLAGS = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_FONT_SELECTED_IN_HDC: enumINKMETRIC_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_ITALIC: enumINKMETRIC_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const IMF_BOLD: enumINKMETRIC_FLAGS = 4i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub type enumRECO_TYPE = i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECO_TYPE_WSTRING: enumRECO_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_TabletPC\"`*"]
pub const RECO_TYPE_WCHAR: enumRECO_TYPE = 1i32;
