#[repr(transparent)]
pub struct IPdfRendererNative(::windows_core::IUnknown);
impl IPdfRendererNative {
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub unsafe fn RenderPageToSurface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGISurface>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::POINT>>(&self, pdfpage: Param0, psurface: Param1, offset: Param2, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RenderPageToSurface)(::windows_core::Interface::as_raw(self), pdfpage.into_param().abi(), psurface.into_param().abi(), offset.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
    #[cfg(feature = "win32-graphics")]
    pub unsafe fn RenderPageToDeviceContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Direct2D::ID2D1DeviceContext>>(&self, pdfpage: Param0, pd2ddevicecontext: Param1, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RenderPageToDeviceContext)(::windows_core::Interface::as_raw(self), pdfpage.into_param().abi(), pd2ddevicecontext.into_param().abi(), ::core::mem::transmute(prenderparams)).ok()
    }
}
impl ::core::convert::From<IPdfRendererNative> for ::windows_core::IUnknown {
    fn from(value: IPdfRendererNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPdfRendererNative> for ::windows_core::IUnknown {
    fn from(value: &IPdfRendererNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPdfRendererNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPdfRendererNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPdfRendererNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPdfRendererNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPdfRendererNative {}
impl ::core::fmt::Debug for IPdfRendererNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPdfRendererNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPdfRendererNative {
    type Vtable = IPdfRendererNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9dcd91_d277_4947_8527_07a0daeda94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "win32-graphics", feature = "win32-graphics"))]
    pub RenderPageToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, psurface: ::windows_core::RawPtr, offset: ::win32_foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-graphics", feature = "win32-graphics")))]
    RenderPageToSurface: usize,
    #[cfg(feature = "win32-graphics")]
    pub RenderPageToDeviceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdfpage: *mut ::core::ffi::c_void, pd2ddevicecontext: ::windows_core::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-graphics"))]
    RenderPageToDeviceContext: usize,
}
#[repr(C)]
#[cfg(feature = "win32-graphics")]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: ::win32_graphics::Direct2D::Common::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: ::win32_graphics::Direct2D::Common::D2D_COLOR_F,
    pub IgnoreHighContrast: ::win32_foundation::BOOLEAN,
}
#[cfg(feature = "win32-graphics")]
impl ::core::marker::Copy for PDF_RENDER_PARAMS {}
#[cfg(feature = "win32-graphics")]
impl ::core::clone::Clone for PDF_RENDER_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "win32-graphics")]
impl ::core::fmt::Debug for PDF_RENDER_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PDF_RENDER_PARAMS").field("SourceRect", &self.SourceRect).field("DestinationWidth", &self.DestinationWidth).field("DestinationHeight", &self.DestinationHeight).field("BackgroundColor", &self.BackgroundColor).field("IgnoreHighContrast", &self.IgnoreHighContrast).finish()
    }
}
#[cfg(feature = "win32-graphics")]
unsafe impl ::windows_core::Abi for PDF_RENDER_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "win32-graphics")]
impl ::core::cmp::PartialEq for PDF_RENDER_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PDF_RENDER_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "win32-graphics")]
impl ::core::cmp::Eq for PDF_RENDER_PARAMS {}
#[cfg(feature = "win32-graphics")]
impl ::core::default::Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "win32-graphics")]
pub type PFN_PDF_CREATE_RENDERER = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::win32_graphics::Dxgi::IDXGIDevice>, param1: *mut ::core::option::Option<IPdfRendererNative>) -> ::windows_core::HRESULT>;
#[cfg(feature = "win32-graphics")]
#[inline]
pub unsafe fn PdfCreateRenderer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGIDevice>>(pdevice: Param0) -> ::windows_core::Result<IPdfRendererNative> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdfCreateRenderer(pdevice: ::windows_core::RawPtr, pprenderer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        PdfCreateRenderer(pdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPdfRendererNative>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
