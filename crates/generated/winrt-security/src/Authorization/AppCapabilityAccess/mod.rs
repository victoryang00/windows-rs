#[repr(transparent)]
pub struct AppCapability(::windows_core::IUnknown);
impl AppCapability {
    pub fn CapabilityName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapabilityName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-system")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn RequestAccessAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<AppCapabilityAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<AppCapabilityAccessStatus>>(result__)
        }
    }
    pub fn CheckAccess(&self) -> ::windows_core::Result<AppCapabilityAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AppCapabilityAccessStatus>::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AppCapabilityAccessStatus>(result__)
        }
    }
    pub fn AccessChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<AppCapability, AppCapabilityAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AccessChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccessChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RequestAccessForCapabilitiesAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(capabilitynames: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForCapabilitiesAsync)(::windows_core::Interface::as_raw(this), capabilitynames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub fn RequestAccessForCapabilitiesForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(user: Param0, capabilitynames: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForCapabilitiesForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), capabilitynames.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    pub fn Create<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(capabilityname: Param0) -> ::windows_core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), capabilityname.into_param().abi(), result__.as_mut_ptr()).from_abi::<AppCapability>(result__)
        })
    }
    #[cfg(feature = "winrt-system")]
    pub fn CreateWithProcessIdForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, capabilityname: Param1, pid: u32) -> ::windows_core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithProcessIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), capabilityname.into_param().abi(), pid, result__.as_mut_ptr()).from_abi::<AppCapability>(result__)
        })
    }
    pub fn IAppCapabilityStatics<R, F: FnOnce(&IAppCapabilityStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AppCapability, IAppCapabilityStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapability {}
impl ::core::fmt::Debug for AppCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapability").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCapability {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapability;{4c49d915-8a2a-4295-9437-2df7c396aff4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCapability {
    type Vtable = IAppCapability_Vtbl;
    const IID: ::windows_core::GUID = <IAppCapability as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCapability {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
}
impl ::core::convert::From<AppCapability> for ::windows_core::IUnknown {
    fn from(value: AppCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapability> for ::windows_core::IUnknown {
    fn from(value: &AppCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCapability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCapability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapability> for ::windows_core::IInspectable {
    fn from(value: AppCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapability> for ::windows_core::IInspectable {
    fn from(value: &AppCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCapability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCapability {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCapability {}
unsafe impl ::core::marker::Sync for AppCapability {}
#[repr(transparent)]
pub struct AppCapabilityAccessChangedEventArgs(::windows_core::IUnknown);
impl AppCapabilityAccessChangedEventArgs {}
impl ::core::clone::Clone for AppCapabilityAccessChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCapabilityAccessChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapabilityAccessChangedEventArgs {}
impl ::core::fmt::Debug for AppCapabilityAccessChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapabilityAccessChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCapabilityAccessChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs;{0a578d15-bdd7-457e-8cca-6f53bd2e5944})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppCapabilityAccessChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppCapabilityAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCapabilityAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCapabilityAccessChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
}
impl ::core::marker::Copy for AppCapabilityAccessStatus {}
impl ::core::clone::Clone for AppCapabilityAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppCapabilityAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AppCapabilityAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCapabilityAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapabilityAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppCapabilityAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapability(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCapability {
    type Vtable = IAppCapability_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapability_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CapabilityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    User: usize,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCapabilityAccessStatus) -> ::windows_core::HRESULT,
    pub AccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityAccessChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppCapabilityStatics {
    type Vtable = IAppCapabilityStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c353e2a_46ee_44e5_af3d_6ad3fc49bd22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub RequestAccessForCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilitynames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RequestAccessForCapabilitiesAsync: usize,
    #[cfg(all(feature = "winrt-foundation", feature = "winrt-system"))]
    pub RequestAccessForCapabilitiesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, capabilitynames: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "winrt-foundation", feature = "winrt-system")))]
    RequestAccessForCapabilitiesForUserAsync: usize,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-system")]
    pub CreateWithProcessIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, capabilityname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, pid: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-system"))]
    CreateWithProcessIdForUser: usize,
}
