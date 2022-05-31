#[repr(transparent)]
pub struct DualSimTile(::windows_core::IUnknown);
impl DualSimTile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DualSimTile, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsPinnedToStart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPinnedToStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn UpdateAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DeleteAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetTileForSim2() -> ::windows_core::Result<DualSimTile> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTileForSim2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DualSimTile>(result__)
        })
    }
    pub fn UpdateDisplayNameForSim1Async<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(name: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateDisplayNameForSim1Async)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateTileUpdaterForSim1() -> ::windows_core::Result<::winrt_ui::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSim1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::TileUpdater>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateTileUpdaterForSim2() -> ::windows_core::Result<::winrt_ui::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSim2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::TileUpdater>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateBadgeUpdaterForSim1() -> ::windows_core::Result<::winrt_ui::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateBadgeUpdaterForSim2() -> ::windows_core::Result<::winrt_ui::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSim2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateToastNotifierForSim1() -> ::windows_core::Result<::winrt_ui::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSim1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::ToastNotifier>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn CreateToastNotifierForSim2() -> ::windows_core::Result<::winrt_ui::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSim2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::ToastNotifier>(result__)
        })
    }
    pub fn IDualSimTileStatics<R, F: FnOnce(&IDualSimTileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DualSimTile, IDualSimTileStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DualSimTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DualSimTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DualSimTile {}
impl ::core::fmt::Debug for DualSimTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DualSimTile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DualSimTile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Phone.StartScreen.DualSimTile;{143ab213-d05f-4041-a18c-3e3fcb75b41e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DualSimTile {
    type Vtable = IDualSimTile_Vtbl;
    const IID: ::windows_core::GUID = <IDualSimTile as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DualSimTile {
    const NAME: &'static str = "Windows.Phone.StartScreen.DualSimTile";
}
impl ::core::convert::From<DualSimTile> for ::windows_core::IUnknown {
    fn from(value: DualSimTile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DualSimTile> for ::windows_core::IUnknown {
    fn from(value: &DualSimTile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DualSimTile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DualSimTile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DualSimTile> for ::windows_core::IInspectable {
    fn from(value: DualSimTile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DualSimTile> for ::windows_core::IInspectable {
    fn from(value: &DualSimTile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DualSimTile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DualSimTile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDualSimTile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDualSimTile {
    type Vtable = IDualSimTile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143ab213_d05f_4041_a18c_3e3fcb75b41e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTile_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPinnedToStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDualSimTileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDualSimTileStatics {
    type Vtable = IDualSimTileStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50567c9e_c58f_4dc9_b6e8_fa6777eeeb37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTileStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetTileForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UpdateDisplayNameForSim1Async: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub CreateTileUpdaterForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateTileUpdaterForSim1: usize,
    #[cfg(feature = "winrt-ui")]
    pub CreateTileUpdaterForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateTileUpdaterForSim2: usize,
    #[cfg(feature = "winrt-ui")]
    pub CreateBadgeUpdaterForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateBadgeUpdaterForSim1: usize,
    #[cfg(feature = "winrt-ui")]
    pub CreateBadgeUpdaterForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateBadgeUpdaterForSim2: usize,
    #[cfg(feature = "winrt-ui")]
    pub CreateToastNotifierForSim1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateToastNotifierForSim1: usize,
    #[cfg(feature = "winrt-ui")]
    pub CreateToastNotifierForSim2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateToastNotifierForSim2: usize,
}
#[repr(transparent)]
pub struct IToastNotificationManagerStatics3(::windows_core::IUnknown);
impl IToastNotificationManagerStatics3 {
    #[cfg(feature = "winrt-ui")]
    pub fn CreateToastNotifierForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<::winrt_ui::Notifications::ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifierForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_ui::Notifications::ToastNotifier>(result__)
        }
    }
}
impl ::core::convert::From<IToastNotificationManagerStatics3> for ::windows_core::IUnknown {
    fn from(value: IToastNotificationManagerStatics3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationManagerStatics3> for ::windows_core::IUnknown {
    fn from(value: &IToastNotificationManagerStatics3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IToastNotificationManagerStatics3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IToastNotificationManagerStatics3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IToastNotificationManagerStatics3> for ::windows_core::IInspectable {
    fn from(value: IToastNotificationManagerStatics3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationManagerStatics3> for ::windows_core::IInspectable {
    fn from(value: &IToastNotificationManagerStatics3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IToastNotificationManagerStatics3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IToastNotificationManagerStatics3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IToastNotificationManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToastNotificationManagerStatics3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationManagerStatics3 {}
impl ::core::fmt::Debug for IToastNotificationManagerStatics3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToastNotificationManagerStatics3").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IToastNotificationManagerStatics3 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2717f54b-50df-4455-8e6e-41e0fc8e13ce}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics3 {
    type Vtable = IToastNotificationManagerStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2717f54b_50df_4455_8e6e_41e0fc8e13ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub CreateToastNotifierForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    CreateToastNotifierForSecondaryTile: usize,
}
