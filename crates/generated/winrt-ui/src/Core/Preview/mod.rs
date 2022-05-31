#[repr(transparent)]
pub struct CoreAppWindowPreview(::windows_core::IUnknown);
impl CoreAppWindowPreview {
    #[cfg(feature = "winrt-ui")]
    pub fn GetIdFromWindow<'a, Param0: ::windows_core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows_core::Result<i32> {
        Self::ICoreAppWindowPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetIdFromWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn ICoreAppWindowPreviewStatics<R, F: FnOnce(&ICoreAppWindowPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CoreAppWindowPreview, ICoreAppWindowPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreAppWindowPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreAppWindowPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreAppWindowPreview {}
impl ::core::fmt::Debug for CoreAppWindowPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreAppWindowPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CoreAppWindowPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.CoreAppWindowPreview;{a4f6e665-365e-5fde-87a5-9543c3a15aa8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
    const IID: ::windows_core::GUID = <ICoreAppWindowPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CoreAppWindowPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.CoreAppWindowPreview";
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows_core::IUnknown {
    fn from(value: CoreAppWindowPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows_core::IUnknown {
    fn from(value: &CoreAppWindowPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreAppWindowPreview> for ::windows_core::IInspectable {
    fn from(value: CoreAppWindowPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAppWindowPreview> for ::windows_core::IInspectable {
    fn from(value: &CoreAppWindowPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CoreAppWindowPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CoreAppWindowPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAppWindowPreview {}
unsafe impl ::core::marker::Sync for CoreAppWindowPreview {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAppWindowPreview {
    type Vtable = ICoreAppWindowPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4f6e665_365e_5fde_87a5_9543c3a15aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreAppWindowPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreAppWindowPreviewStatics {
    type Vtable = ICoreAppWindowPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ac21be_423b_5db6_8a8e_4dc87353b75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAppWindowPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub GetIdFromWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetIdFromWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d00de1_cbe5_4f31_8414_361da046518f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec5f0488_6425_4777_a536_cb5634427f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreview_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCloseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemNavigationManagerPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemNavigationManagerPreviewStatics {
    type Vtable = ISystemNavigationManagerPreviewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e971360_df74_4bce_84cb_bd1181ac0a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(::windows_core::IUnknown);
impl SystemNavigationCloseRequestedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemNavigationCloseRequestedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationCloseRequestedPreviewEventArgs {}
impl ::core::fmt::Debug for SystemNavigationCloseRequestedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationCloseRequestedPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemNavigationCloseRequestedPreviewEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs;{83d00de1-cbe5-4f31-8414-361da046518f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemNavigationCloseRequestedPreviewEventArgs {
    type Vtable = ISystemNavigationCloseRequestedPreviewEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISystemNavigationCloseRequestedPreviewEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemNavigationCloseRequestedPreviewEventArgs {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationCloseRequestedPreviewEventArgs";
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemNavigationCloseRequestedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationCloseRequestedPreviewEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SystemNavigationCloseRequestedPreviewEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemNavigationCloseRequestedPreviewEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemNavigationCloseRequestedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for SystemNavigationCloseRequestedPreviewEventArgs {}
#[repr(transparent)]
pub struct SystemNavigationManagerPreview(::windows_core::IUnknown);
impl SystemNavigationManagerPreview {
    pub fn CloseRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CloseRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCloseRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCloseRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<SystemNavigationManagerPreview> {
        Self::ISystemNavigationManagerPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemNavigationManagerPreview>(result__)
        })
    }
    pub fn ISystemNavigationManagerPreviewStatics<R, F: FnOnce(&ISystemNavigationManagerPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemNavigationManagerPreview, ISystemNavigationManagerPreviewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemNavigationManagerPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemNavigationManagerPreview {}
impl ::core::fmt::Debug for SystemNavigationManagerPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemNavigationManagerPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemNavigationManagerPreview {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.Preview.SystemNavigationManagerPreview;{ec5f0488-6425-4777-a536-cb5634427f0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemNavigationManagerPreview {
    type Vtable = ISystemNavigationManagerPreview_Vtbl;
    const IID: ::windows_core::GUID = <ISystemNavigationManagerPreview as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemNavigationManagerPreview {
    const NAME: &'static str = "Windows.UI.Core.Preview.SystemNavigationManagerPreview";
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows_core::IUnknown {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows_core::IUnknown {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemNavigationManagerPreview> for ::windows_core::IInspectable {
    fn from(value: SystemNavigationManagerPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemNavigationManagerPreview> for ::windows_core::IInspectable {
    fn from(value: &SystemNavigationManagerPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemNavigationManagerPreview {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemNavigationManagerPreview {}
unsafe impl ::core::marker::Sync for SystemNavigationManagerPreview {}
