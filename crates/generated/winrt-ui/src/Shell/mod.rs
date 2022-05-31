pub struct AdaptiveCardBuilder;
impl AdaptiveCardBuilder {
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(value: Param0) -> ::windows_core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IAdaptiveCard>(result__)
        })
    }
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[repr(transparent)]
pub struct IAdaptiveCard(::windows_core::IUnknown);
impl IAdaptiveCard {
    pub fn ToJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToJson)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows_core::IUnknown {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows_core::IUnknown {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAdaptiveCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAdaptiveCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdaptiveCard> for ::windows_core::IInspectable {
    fn from(value: IAdaptiveCard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCard> for ::windows_core::IInspectable {
    fn from(value: &IAdaptiveCard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAdaptiveCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAdaptiveCard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdaptiveCard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCard {}
impl ::core::fmt::Debug for IAdaptiveCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCard").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCard_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72d0568c_a274_41cd_82a8_989d40b9b05e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCard_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(::windows_core::IUnknown);
impl IAdaptiveCardBuilderStatics {
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAdaptiveCardFromJson)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IAdaptiveCard>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows_core::IUnknown {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows_core::IUnknown {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdaptiveCardBuilderStatics> for ::windows_core::IInspectable {
    fn from(value: IAdaptiveCardBuilderStatics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveCardBuilderStatics> for ::windows_core::IInspectable {
    fn from(value: &IAdaptiveCardBuilderStatics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAdaptiveCardBuilderStatics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdaptiveCardBuilderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveCardBuilderStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveCardBuilderStatics {}
impl ::core::fmt::Debug for IAdaptiveCardBuilderStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveCardBuilderStatics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecurityAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96ac500c_aed4_561d_bde8_953520343a2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, detailsuri: ::windows_core::RawPtr, registerperuser: bool, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UpdateState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: SecurityAppKind, guidregistration: ::windows_core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4578dc09_a523_5756_a995_e4feb991fff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WindowId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowId) -> ::windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ShareWindowCommand) -> ::windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ShareWindowCommand) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCommandChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CommandRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCommandRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CommandInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCommandInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShareWindowCommandSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShareWindowCommandSourceStatics {
    type Vtable = IShareWindowCommandSourceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0eb6656_9cac_517c_b6c7_8ef715084295);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPinningAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCurrentAppPinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Core")]
    pub IsAppListEntryPinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    IsAppListEntryPinnedAsync: usize,
    pub RequestPinCurrentAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Core")]
    pub RequestPinAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    RequestPinAppListEntryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITaskbarManager2 {
    type Vtable = ITaskbarManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSecondaryTilePinnedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_StartScreen")]
    pub RequestPinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondarytile: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_StartScreen"))]
    RequestPinSecondaryTileAsync: usize,
    pub TryUnpinSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITaskbarManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl ::core::marker::Copy for SecurityAppKind {}
impl ::core::clone::Clone for SecurityAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SecurityAppKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SecurityAppManager(::windows_core::IUnknown);
impl SecurityAppManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SecurityAppManager, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Register<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, kind: SecurityAppKind, displayname: Param1, detailsuri: Param2, registerperuser: bool) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(this), kind, displayname.into_param().abi(), detailsuri.into_param().abi(), registerperuser, result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Unregister<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, kind: SecurityAppKind, guidregistration: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), kind, guidregistration.into_param().abi()).ok() }
    }
    pub fn UpdateState<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param4: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, kind: SecurityAppKind, guidregistration: Param1, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: Param4) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateState)(::windows_core::Interface::as_raw(this), kind, guidregistration.into_param().abi(), state, substatus, detailsuri.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SecurityAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecurityAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecurityAppManager {}
impl ::core::fmt::Debug for SecurityAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
    const IID: ::windows_core::GUID = <ISecurityAppManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
impl ::core::convert::From<SecurityAppManager> for ::windows_core::IUnknown {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows_core::IUnknown {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SecurityAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SecurityAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SecurityAppManager> for ::windows_core::IInspectable {
    fn from(value: SecurityAppManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SecurityAppManager> for ::windows_core::IInspectable {
    fn from(value: &SecurityAppManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SecurityAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SecurityAppManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SecurityAppManager {}
unsafe impl ::core::marker::Sync for SecurityAppManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for SecurityAppState {}
impl ::core::clone::Clone for SecurityAppState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SecurityAppState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl ::core::marker::Copy for SecurityAppSubstatus {}
impl ::core::clone::Clone for SecurityAppSubstatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SecurityAppSubstatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SecurityAppSubstatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SecurityAppSubstatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecurityAppSubstatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareWindowCommand {}
impl ::core::clone::Clone for ShareWindowCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShareWindowCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ShareWindowCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShareWindowCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(::windows_core::IUnknown);
impl ShareWindowCommandEventArgs {
    pub fn WindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowId>::zeroed();
            (::windows_core::Interface::vtable(this).WindowId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WindowId>(result__)
        }
    }
    pub fn Command(&self) -> ::windows_core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ShareWindowCommand>::zeroed();
            (::windows_core::Interface::vtable(this).Command)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ShareWindowCommand>(result__)
        }
    }
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommand)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ShareWindowCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandEventArgs {}
impl ::core::fmt::Debug for ShareWindowCommandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IShareWindowCommandEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows_core::IUnknown {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareWindowCommandEventArgs> for ::windows_core::IInspectable {
    fn from(value: ShareWindowCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ShareWindowCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ShareWindowCommandEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandEventArgs {}
unsafe impl ::core::marker::Sync for ShareWindowCommandEventArgs {}
#[repr(transparent)]
pub struct ShareWindowCommandSource(::windows_core::IUnknown);
impl ShareWindowCommandSource {
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCommandChanged(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCommandChanged)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CommandRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CommandRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCommandRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCommandRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CommandInvoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CommandInvoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCommandInvoked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCommandInvoked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ShareWindowCommandSource>(result__)
        })
    }
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ShareWindowCommandSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareWindowCommandSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareWindowCommandSource {}
impl ::core::fmt::Debug for ShareWindowCommandSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareWindowCommandSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
    const IID: ::windows_core::GUID = <IShareWindowCommandSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows_core::IUnknown {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows_core::IUnknown {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ShareWindowCommandSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareWindowCommandSource> for ::windows_core::IInspectable {
    fn from(value: ShareWindowCommandSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareWindowCommandSource> for ::windows_core::IInspectable {
    fn from(value: &ShareWindowCommandSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ShareWindowCommandSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ShareWindowCommandSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShareWindowCommandSource {}
unsafe impl ::core::marker::Sync for ShareWindowCommandSource {}
#[repr(transparent)]
pub struct TaskbarManager(::windows_core::IUnknown);
impl TaskbarManager {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPinningAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPinningAllowed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAppPinnedAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn IsAppListEntryPinnedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsAppListEntryPinnedAsync)(::windows_core::Interface::as_raw(this), applistentry.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinCurrentAppAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn RequestPinAppListEntryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinAppListEntryAsync)(::windows_core::Interface::as_raw(this), applistentry.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn IsSecondaryTilePinnedAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSecondaryTilePinnedAsync)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "UI_StartScreen")]
    pub fn RequestPinSecondaryTileAsync<'a, Param0: ::windows_core::IntoParam<'a, super::StartScreen::SecondaryTile>>(&self, secondarytile: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestPinSecondaryTileAsync)(::windows_core::Interface::as_raw(this), secondarytile.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn TryUnpinSecondaryTileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUnpinSecondaryTileAsync)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TaskbarManager>(result__)
        })
    }
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TaskbarManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TaskbarManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TaskbarManager {}
impl ::core::fmt::Debug for TaskbarManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TaskbarManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
    const IID: ::windows_core::GUID = <ITaskbarManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
impl ::core::convert::From<TaskbarManager> for ::windows_core::IUnknown {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows_core::IUnknown {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TaskbarManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TaskbarManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TaskbarManager> for ::windows_core::IInspectable {
    fn from(value: TaskbarManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TaskbarManager> for ::windows_core::IInspectable {
    fn from(value: &TaskbarManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TaskbarManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TaskbarManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TaskbarManager {}
unsafe impl ::core::marker::Sync for TaskbarManager {}
