pub type CoreIncrementalInkStroke = *mut ::core::ffi::c_void;
pub type CoreInkIndependentInputSource = *mut ::core::ffi::c_void;
pub type CoreInkPresenterHost = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Input_Inking_Core\"`*"]
#[repr(transparent)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWetStrokeDisposition {}
impl ::core::clone::Clone for CoreWetStrokeDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreWetStrokeUpdateEventArgs = *mut ::core::ffi::c_void;
pub type CoreWetStrokeUpdateSource = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICoreIncrementalInkStroke {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendInkPoints: unsafe extern "system" fn(this: *mut *mut Self, inkpoints: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendInkPoints: usize,
    pub CreateInkStroke: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DrawingAttributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PointTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PointTransform: usize,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
}
impl ::windows_sys::core::Interface for ICoreIncrementalInkStroke {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4255126995, data2: 40294, data3: 20349, data4: [165, 127, 204, 112, 185, 207, 170, 118] };
}
#[repr(C)]
pub struct ICoreIncrementalInkStrokeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, drawingattributes: *mut ::core::ffi::c_void, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ICoreIncrementalInkStrokeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3620052806, data2: 36264, data3: 20336, data4: [151, 81, 229, 59, 182, 223, 69, 150] };
}
#[repr(C)]
pub struct ICoreInkIndependentInputSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerEntering: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerEntering: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerEntering: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerEntering: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerHovering: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerHovering: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerHovering: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerHovering: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerExiting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerExiting: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerExiting: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerExiting: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerPressing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerPressing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerPressing: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerPressing: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerMoving: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerMoving: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerMoving: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerMoving: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerReleasing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerReleasing: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerReleasing: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerReleasing: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub PointerLost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))]
    PointerLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePointerLost: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePointerLost: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInkIndependentInputSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 968068521, data2: 30265, data3: 17561, data4: [165, 181, 25, 29, 0, 227, 91, 22] };
}
#[repr(C)]
pub struct ICoreInkIndependentInputSource2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Core")]
    pub PointerCursor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerCursor: usize,
    #[cfg(feature = "UI_Core")]
    pub SetPointerCursor: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetPointerCursor: usize,
}
impl ::windows_sys::core::Interface for ICoreInkIndependentInputSource2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 675721234, data2: 2905, data3: 23481, data4: [163, 197, 190, 203, 124, 240, 58, 51] };
}
#[repr(C)]
pub struct ICoreInkIndependentInputSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreInkIndependentInputSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1944453403, data2: 32960, data3: 19963, data4: [155, 102, 16, 186, 127, 63, 156, 132] };
}
#[repr(C)]
pub struct ICoreInkPresenterHost {
    pub base__: ::windows_sys::core::IInspectable,
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub RootVisual: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    RootVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetRootVisual: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetRootVisual: usize,
}
impl ::windows_sys::core::Interface for ICoreInkPresenterHost {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 963545574, data2: 32085, data3: 17943, data4: [158, 88, 104, 199, 12, 145, 105, 185] };
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub NewInkPoints: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NewInkPoints: usize,
    pub PointerId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Disposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CoreWetStrokeDisposition) -> ::windows_sys::core::HRESULT,
    pub SetDisposition: unsafe extern "system" fn(this: *mut *mut Self, value: CoreWetStrokeDisposition) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWetStrokeUpdateEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4211593548, data2: 13184, data3: 17786, data4: [169, 135, 153, 19, 87, 137, 108, 27] };
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateSource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub WetStrokeStarting: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WetStrokeStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWetStrokeStarting: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWetStrokeStarting: usize,
    #[cfg(feature = "Foundation")]
    pub WetStrokeContinuing: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WetStrokeContinuing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWetStrokeContinuing: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWetStrokeContinuing: usize,
    #[cfg(feature = "Foundation")]
    pub WetStrokeStopping: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WetStrokeStopping: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWetStrokeStopping: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWetStrokeStopping: usize,
    #[cfg(feature = "Foundation")]
    pub WetStrokeCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WetStrokeCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWetStrokeCompleted: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWetStrokeCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub WetStrokeCanceled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WetStrokeCanceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWetStrokeCanceled: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWetStrokeCanceled: usize,
    pub InkPresenter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWetStrokeUpdateSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 527535650, data2: 61010, data3: 19968, data4: [130, 9, 76, 62, 91, 33, 163, 204] };
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreWetStrokeUpdateSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1034788026, data2: 7485, data3: 18094, data4: [171, 157, 134, 71, 72, 108, 111, 144] };
}
