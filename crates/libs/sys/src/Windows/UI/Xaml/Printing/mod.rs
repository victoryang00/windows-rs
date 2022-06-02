pub type AddPagesEventArgs = *mut ::core::ffi::c_void;
pub type AddPagesEventHandler = *mut ::core::ffi::c_void;
pub type GetPreviewPageEventArgs = *mut ::core::ffi::c_void;
pub type GetPreviewPageEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAddPagesEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
}
#[repr(C)]
pub struct IGetPreviewPageEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PageNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPaginateEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintDocument {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_Printing")]
    pub DocumentSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    DocumentSource: usize,
    #[cfg(feature = "Foundation")]
    pub Paginate: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paginate: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaginate: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaginate: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewPage: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewPage: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGetPreviewPage: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGetPreviewPage: usize,
    #[cfg(feature = "Foundation")]
    pub AddPages: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPages: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAddPages: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAddPages: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut *mut Self, pagevisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddPagesComplete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SetPreviewPageCount: unsafe extern "system" fn(this: *mut *mut Self, count: i32, r#type: PreviewPageCountType) -> ::windows_sys::core::HRESULT,
    pub SetPreviewPage: unsafe extern "system" fn(this: *mut *mut Self, pagenumber: i32, pagevisual: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InvalidatePreview: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintDocumentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPrintDocumentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub DocumentSourceProperty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type PaginateEventArgs = *mut ::core::ffi::c_void;
pub type PaginateEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: Self = Self(0i32);
    pub const Intermediate: Self = Self(1i32);
}
impl ::core::marker::Copy for PreviewPageCountType {}
impl ::core::clone::Clone for PreviewPageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PrintDocument = *mut ::core::ffi::c_void;
