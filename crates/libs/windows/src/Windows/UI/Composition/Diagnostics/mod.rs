#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CompositionDebugHeatMaps(::windows_core::IUnknown);
impl CompositionDebugHeatMaps {
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn Hide<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this), subtree.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn ShowMemoryUsage<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowMemoryUsage)(::windows_core::Interface::as_raw(this), subtree.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn ShowOverdraw<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, subtree: Param0, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowOverdraw)(::windows_core::Interface::as_raw(this), subtree.into_param().abi(), contentkinds).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn ShowRedraw<'a, Param0: ::windows_core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowRedraw)(::windows_core::Interface::as_raw(this), subtree.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CompositionDebugHeatMaps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionDebugHeatMaps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionDebugHeatMaps {}
impl ::core::fmt::Debug for CompositionDebugHeatMaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugHeatMaps").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps;{e49c90ac-2ff3-5805-718c-b725ee07650f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_Vtbl;
    const IID: ::windows_core::GUID = <ICompositionDebugHeatMaps as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
impl ::core::convert::From<CompositionDebugHeatMaps> for ::windows_core::IUnknown {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionDebugHeatMaps> for ::windows_core::IUnknown {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionDebugHeatMaps> for ::windows_core::IInspectable {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionDebugHeatMaps> for ::windows_core::IInspectable {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompositionDebugHeatMaps {}
unsafe impl ::core::marker::Sync for CompositionDebugHeatMaps {}
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for CompositionDebugOverdrawContentKinds {}
impl ::core::clone::Clone for CompositionDebugOverdrawContentKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompositionDebugOverdrawContentKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CompositionDebugOverdrawContentKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompositionDebugOverdrawContentKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugOverdrawContentKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CompositionDebugSettings(::windows_core::IUnknown);
impl CompositionDebugSettings {
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn HeatMaps(&self) -> ::windows_core::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeatMaps)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CompositionDebugHeatMaps>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
    pub fn TryGetSettings<'a, Param0: ::windows_core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows_core::Result<CompositionDebugSettings> {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetSettings)(::windows_core::Interface::as_raw(this), compositor.into_param().abi(), result__.as_mut_ptr()).from_abi::<CompositionDebugSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompositionDebugSettingsStatics<R, F: FnOnce(&ICompositionDebugSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CompositionDebugSettings, ICompositionDebugSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositionDebugSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionDebugSettings {}
impl ::core::fmt::Debug for CompositionDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugSettings;{2831987e-1d82-4d38-b7b7-efd11c7bc3d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICompositionDebugSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugSettings";
}
impl ::core::convert::From<CompositionDebugSettings> for ::windows_core::IUnknown {
    fn from(value: CompositionDebugSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionDebugSettings> for ::windows_core::IUnknown {
    fn from(value: &CompositionDebugSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CompositionDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CompositionDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionDebugSettings> for ::windows_core::IInspectable {
    fn from(value: CompositionDebugSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionDebugSettings> for ::windows_core::IInspectable {
    fn from(value: &CompositionDebugSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CompositionDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CompositionDebugSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CompositionDebugSettings {}
unsafe impl ::core::marker::Sync for CompositionDebugSettings {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugHeatMaps(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe49c90ac_2ff3_5805_718c_b725ee07650f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowOverdraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: ::windows_core::RawPtr, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows_core::HRESULT,
    pub ShowRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2831987e_1d82_4d38_b7b7_efd11c7bc3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeatMaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionDebugSettingsStatics {
    type Vtable = ICompositionDebugSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64ec1f1e_6af8_4af8_b814_c870fd5a9505);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
