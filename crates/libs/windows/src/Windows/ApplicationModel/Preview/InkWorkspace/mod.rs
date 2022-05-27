#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkWorkspaceHostedAppManagerStatics {
    type Vtable = IInkWorkspaceHostedAppManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`*"]
#[repr(transparent)]
pub struct InkWorkspaceHostedAppManager(::windows_core::IUnknown);
impl InkWorkspaceHostedAppManager {
    #[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetThumbnailAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Preview_InkWorkspace\"`*"]
    pub fn GetForCurrentApp() -> ::windows_core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentApp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InkWorkspaceHostedAppManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkWorkspaceHostedAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkWorkspaceHostedAppManager {}
impl ::core::fmt::Debug for InkWorkspaceHostedAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkWorkspaceHostedAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager;{fe0a7990-5e59-4bb7-8a63-7d218cd96300})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: ::windows_core::GUID = <IInkWorkspaceHostedAppManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
impl ::core::convert::From<InkWorkspaceHostedAppManager> for ::windows_core::IUnknown {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkWorkspaceHostedAppManager> for ::windows_core::IUnknown {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkWorkspaceHostedAppManager> for ::windows_core::IInspectable {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkWorkspaceHostedAppManager> for ::windows_core::IInspectable {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::core::marker::Sync for InkWorkspaceHostedAppManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
