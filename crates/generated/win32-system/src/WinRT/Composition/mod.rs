#[repr(transparent)]
pub struct ICompositionCapabilitiesInteropFactory(::windows_core::IUnknown);
impl ICompositionCapabilitiesInteropFactory {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwnd: Param0) -> ::windows_core::Result<::winrt_ui::Composition::CompositionCapabilities> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_ui::Composition::CompositionCapabilities>(result__)
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows_core::IUnknown {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows_core::IUnknown {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows_core::IInspectable {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows_core::IInspectable {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionCapabilitiesInteropFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionCapabilitiesInteropFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionCapabilitiesInteropFactory {}
impl ::core::fmt::Debug for ICompositionCapabilitiesInteropFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionCapabilitiesInteropFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositionCapabilitiesInteropFactory {
    type Vtable = ICompositionCapabilitiesInteropFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: ::win32_foundation::HWND, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetForWindow: usize,
}
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop(::windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    pub unsafe fn BeginDraw(&self, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(::windows_core::Interface::as_raw(self), sizepixels.into_param().abi()).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop> for ::windows_core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop> for ::windows_core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionDrawingSurfaceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionDrawingSurfaceInterop {}
impl ::core::fmt::Debug for ICompositionDrawingSurfaceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionDrawingSurfaceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositionDrawingSurfaceInterop {
    type Vtable = ICompositionDrawingSurfaceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizepixels: ::win32_foundation::SIZE) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop2(::windows_core::IUnknown);
impl ICompositionDrawingSurfaceInterop2 {
    pub unsafe fn BeginDraw(&self, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Resize)(::windows_core::Interface::as_raw(self), sizepixels.into_param().abi()).ok()
    }
    pub unsafe fn Scroll(&self, scrollrect: *const ::win32_foundation::RECT, cliprect: *const ::win32_foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Scroll)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CopySurface<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, destinationresource: Param0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopySurface)(::windows_core::Interface::as_raw(self), destinationresource.into_param().abi(), ::core::mem::transmute(destinationoffsetx), ::core::mem::transmute(destinationoffsety), ::core::mem::transmute(sourcerectangle)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ::windows_core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ::windows_core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICompositionDrawingSurfaceInterop> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICompositionDrawingSurfaceInterop> for &'a ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows_core::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionDrawingSurfaceInterop2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionDrawingSurfaceInterop2 {}
impl ::core::fmt::Debug for ICompositionDrawingSurfaceInterop2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionDrawingSurfaceInterop2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositionDrawingSurfaceInterop2 {
    type Vtable = ICompositionDrawingSurfaceInterop2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_Vtbl {
    pub base__: ICompositionDrawingSurfaceInterop_Vtbl,
    pub CopySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICompositionGraphicsDeviceInterop(::windows_core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    pub unsafe fn GetRenderingDevice(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetRenderingDevice)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    pub unsafe fn SetRenderingDevice<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRenderingDevice)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICompositionGraphicsDeviceInterop> for ::windows_core::IUnknown {
    fn from(value: ICompositionGraphicsDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionGraphicsDeviceInterop> for ::windows_core::IUnknown {
    fn from(value: &ICompositionGraphicsDeviceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionGraphicsDeviceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionGraphicsDeviceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionGraphicsDeviceInterop {}
impl ::core::fmt::Debug for ICompositionGraphicsDeviceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionGraphicsDeviceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositionGraphicsDeviceInterop {
    type Vtable = ICompositionGraphicsDeviceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICompositorDesktopInterop(::windows_core::IUnknown);
impl ICompositorDesktopInterop {
    #[cfg(feature = "UI_Composition_Desktop")]
    pub unsafe fn CreateDesktopWindowTarget<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, hwndtarget: Param0, istopmost: Param1) -> ::windows_core::Result<::winrt_ui::Composition::Desktop::DesktopWindowTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateDesktopWindowTarget)(::windows_core::Interface::as_raw(self), hwndtarget.into_param().abi(), istopmost.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_ui::Composition::Desktop::DesktopWindowTarget>(result__)
    }
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnsureOnThread)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(threadid)).ok()
    }
}
impl ::core::convert::From<ICompositorDesktopInterop> for ::windows_core::IUnknown {
    fn from(value: ICompositorDesktopInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorDesktopInterop> for ::windows_core::IUnknown {
    fn from(value: &ICompositorDesktopInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositorDesktopInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositorDesktopInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositorDesktopInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositorDesktopInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositorDesktopInterop {}
impl ::core::fmt::Debug for ICompositorDesktopInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositorDesktopInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29e691fa_4567_4dca_b319_d0f207eb6807);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "UI_Composition_Desktop")]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndtarget: ::win32_foundation::HWND, istopmost: ::win32_foundation::BOOL, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_Desktop"))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICompositorInterop(::windows_core::IUnknown);
impl ICompositorInterop {
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, swapchain: Param0) -> ::windows_core::Result<::winrt_ui::Composition::ICompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateCompositionSurfaceForHandle)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_ui::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, swapchain: Param0) -> ::windows_core::Result<::winrt_ui::Composition::ICompositionSurface> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateCompositionSurfaceForSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_ui::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows_core::Result<::winrt_ui::Composition::CompositionGraphicsDevice> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateGraphicsDevice)(::windows_core::Interface::as_raw(self), renderingdevice.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::winrt_ui::Composition::CompositionGraphicsDevice>(result__)
    }
}
impl ::core::convert::From<ICompositorInterop> for ::windows_core::IUnknown {
    fn from(value: ICompositorInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorInterop> for ::windows_core::IUnknown {
    fn from(value: &ICompositorInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICompositorInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICompositorInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositorInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositorInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositorInterop {}
impl ::core::fmt::Debug for ICompositorInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositorInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ICompositorInterop {
    type Vtable = ICompositorInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::win32_foundation::HANDLE, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForHandle: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateGraphicsDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateGraphicsDevice: usize,
}
#[repr(transparent)]
pub struct IDesktopWindowTargetInterop(::windows_core::IUnknown);
impl IDesktopWindowTargetInterop {
    pub unsafe fn Hwnd(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).Hwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IDesktopWindowTargetInterop> for ::windows_core::IUnknown {
    fn from(value: IDesktopWindowTargetInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowTargetInterop> for ::windows_core::IUnknown {
    fn from(value: &IDesktopWindowTargetInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowTargetInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowTargetInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowTargetInterop {}
impl ::core::fmt::Debug for IDesktopWindowTargetInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowTargetInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDesktopWindowTargetInterop {
    type Vtable = IDesktopWindowTargetInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISwapChainInterop(::windows_core::IUnknown);
impl ISwapChainInterop {
    pub unsafe fn SetSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, swapchain: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainInterop> for ::windows_core::IUnknown {
    fn from(value: ISwapChainInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainInterop> for ::windows_core::IUnknown {
    fn from(value: &ISwapChainInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISwapChainInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISwapChainInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainInterop {}
impl ::core::fmt::Debug for ISwapChainInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISwapChainInterop {
    type Vtable = ISwapChainInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26f496a0_7f38_45fb_88f7_faaabe67dd59);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IVisualInteractionSourceInterop(::windows_core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const ::win32_ui::Input::Pointer::POINTER_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TryRedirectForManipulation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pointerinfo)).ok()
    }
}
impl ::core::convert::From<IVisualInteractionSourceInterop> for ::windows_core::IUnknown {
    fn from(value: IVisualInteractionSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualInteractionSourceInterop> for ::windows_core::IUnknown {
    fn from(value: &IVisualInteractionSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualInteractionSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualInteractionSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualInteractionSourceInterop {}
impl ::core::fmt::Debug for IVisualInteractionSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualInteractionSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IVisualInteractionSourceInterop {
    type Vtable = IVisualInteractionSourceInterop_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerinfo: *const ::win32_ui::Input::Pointer::POINTER_INFO) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
