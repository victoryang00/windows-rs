pub type CompositeTransform3D = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ICompositeTransform3D {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub CenterZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub ScaleZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub TranslateZ: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetTranslateZ: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICompositeTransform3DStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CenterXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CenterZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ScaleZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TranslateZProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMatrix3DHelper {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IMatrix3DHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Identity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Matrix3D) -> ::windows_sys::core::HRESULT,
    pub Multiply: unsafe extern "system" fn(this: *mut *mut Self, matrix1: Matrix3D, matrix2: Matrix3D, result__: *mut Matrix3D) -> ::windows_sys::core::HRESULT,
    pub FromElements: unsafe extern "system" fn(this: *mut *mut Self, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64, result__: *mut Matrix3D) -> ::windows_sys::core::HRESULT,
    pub GetHasInverse: unsafe extern "system" fn(this: *mut *mut Self, target: Matrix3D, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(this: *mut *mut Self, target: Matrix3D, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Invert: unsafe extern "system" fn(this: *mut *mut Self, target: Matrix3D, result__: *mut Matrix3D) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPerspectiveTransform3D {
    pub base__: ::windows_sys::core::IInspectable,
    pub Depth: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetDepth: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OffsetX: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
    pub OffsetY: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut *mut Self, value: f64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPerspectiveTransform3DStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DepthProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffsetXProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OffsetYProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ITransform3D {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct ITransform3DFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Media_Media3D\"`*"]
pub struct Matrix3D {
    pub M11: f64,
    pub M12: f64,
    pub M13: f64,
    pub M14: f64,
    pub M21: f64,
    pub M22: f64,
    pub M23: f64,
    pub M24: f64,
    pub M31: f64,
    pub M32: f64,
    pub M33: f64,
    pub M34: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
    pub OffsetZ: f64,
    pub M44: f64,
}
impl ::core::marker::Copy for Matrix3D {}
impl ::core::clone::Clone for Matrix3D {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Matrix3DHelper = *mut ::core::ffi::c_void;
pub type PerspectiveTransform3D = *mut ::core::ffi::c_void;
pub type Transform3D = *mut ::core::ffi::c_void;
