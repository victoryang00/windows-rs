pub type Ellipse = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IEllipse {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ILine {
    pub base__: ::windows_sys::core::IInspectable,
    pub X1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetX1: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Y1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetY1: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub X2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetX2: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub Y2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetY2: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub X1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Y1Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub X2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Y2Property: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPath {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Data: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Data: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetData: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetData: usize,
}
#[repr(C)]
pub struct IPathFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPathStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DataProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPolygon {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FillRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FillRule: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFillRule: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFillRule: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Points: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetPoints: usize,
}
#[repr(C)]
pub struct IPolygonStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPolyline {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FillRule: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FillRule: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFillRule: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::FillRule) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFillRule: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub Points: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    Points: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetPoints: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetPoints: usize,
}
#[repr(C)]
pub struct IPolylineStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillRuleProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub PointsProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRectangle {
    pub base__: ::windows_sys::core::IInspectable,
    pub RadiusX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRectangleStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RadiusXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IShape {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stroke: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stroke: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStroke: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStroke: usize,
    pub StrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeStartLineCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeStartLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeStartLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeEndLineCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeEndLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeEndLineCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::PenLineJoin) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeLineJoin: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::PenLineJoin) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeLineJoin: usize,
    pub StrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StrokeDashCap: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::PenLineCap) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStrokeDashCap: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub StrokeDashArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    StrokeDashArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media"))]
    pub SetStrokeDashArray: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media")))]
    SetStrokeDashArray: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(this: *mut *mut Self, value: super::Media::Stretch) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub GeometryTransform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    GeometryTransform: usize,
}
#[repr(C)]
pub struct IShape2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "UI_Composition")]
    pub GetAlphaMask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetAlphaMask: usize,
}
#[repr(C)]
pub struct IShapeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IShapeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub FillProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeMiterLimitProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeThicknessProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeStartLineCapProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeEndLineCapProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeLineJoinProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeDashOffsetProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeDashCapProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StrokeDashArrayProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type Line = *mut ::core::ffi::c_void;
pub type Path = *mut ::core::ffi::c_void;
pub type Polygon = *mut ::core::ffi::c_void;
pub type Polyline = *mut ::core::ffi::c_void;
pub type Rectangle = *mut ::core::ffi::c_void;
pub type Shape = *mut ::core::ffi::c_void;
