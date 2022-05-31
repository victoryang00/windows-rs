#[cfg(feature = "Core")]
pub mod Core;
#[repr(transparent)]
pub struct AdvancedColorInfo(::windows_core::IUnknown);
impl AdvancedColorInfo {
    pub fn CurrentAdvancedColorKind(&self) -> ::windows_core::Result<AdvancedColorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdvancedColorKind>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentAdvancedColorKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdvancedColorKind>(result__)
        }
    }
    pub fn RedPrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).RedPrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn GreenPrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GreenPrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn BluePrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).BluePrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn WhitePoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).WhitePoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MinLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SdrWhiteLevelInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).SdrWhiteLevelInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsHdrMetadataFormatCurrentlySupported)(::windows_core::Interface::as_raw(this), format, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsAdvancedColorKindAvailable)(::windows_core::Interface::as_raw(this), kind, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedColorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvancedColorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvancedColorInfo {}
impl ::core::fmt::Debug for AdvancedColorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedColorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdvancedColorInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.AdvancedColorInfo;{8797dcfb-b229-4081-ae9a-2cc85e34ad6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAdvancedColorInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.AdvancedColorInfo";
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows_core::IUnknown {
    fn from(value: AdvancedColorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows_core::IUnknown {
    fn from(value: &AdvancedColorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdvancedColorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedColorInfo> for ::windows_core::IInspectable {
    fn from(value: AdvancedColorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedColorInfo> for ::windows_core::IInspectable {
    fn from(value: &AdvancedColorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdvancedColorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdvancedColorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedColorInfo {}
unsafe impl ::core::marker::Sync for AdvancedColorInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl ::core::marker::Copy for AdvancedColorKind {}
impl ::core::clone::Clone for AdvancedColorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdvancedColorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdvancedColorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdvancedColorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvancedColorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdvancedColorKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.AdvancedColorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct BrightnessOverride(::windows_core::IUnknown);
impl BrightnessOverride {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOverrideActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverrideActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn BrightnessLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightnessLevel)(::windows_core::Interface::as_raw(this), brightnesslevel, options).ok() }
    }
    pub fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightnessScenario)(::windows_core::Interface::as_raw(this), scenario, options).ok() }
    }
    pub fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetLevelForScenario)(::windows_core::Interface::as_raw(this), scenario, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn StartOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BrightnessOverride, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsSupportedChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BrightnessOverride, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverrideActiveChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn BrightnessLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<BrightnessOverride, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBrightnessLevelChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBrightnessLevelChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefaultForSystem() -> ::windows_core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultForSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BrightnessOverride>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BrightnessOverride>(result__)
        })
    }
    pub fn SaveForSystemAsync<'a, Param0: ::windows_core::IntoParam<'a, BrightnessOverride>>(value: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveForSystemAsync)(::windows_core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IBrightnessOverrideStatics<R, F: FnOnce(&IBrightnessOverrideStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BrightnessOverride, IBrightnessOverrideStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BrightnessOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BrightnessOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BrightnessOverride {}
impl ::core::fmt::Debug for BrightnessOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrightnessOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BrightnessOverride {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverride;{96c9621a-c143-4392-bedd-4a7e9574c8fd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BrightnessOverride {
    type Vtable = IBrightnessOverride_Vtbl;
    const IID: ::windows_core::GUID = <IBrightnessOverride as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverride";
}
impl ::core::convert::From<BrightnessOverride> for ::windows_core::IUnknown {
    fn from(value: BrightnessOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows_core::IUnknown {
    fn from(value: &BrightnessOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BrightnessOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BrightnessOverride> for ::windows_core::IInspectable {
    fn from(value: BrightnessOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverride> for ::windows_core::IInspectable {
    fn from(value: &BrightnessOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BrightnessOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BrightnessOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BrightnessOverride {}
unsafe impl ::core::marker::Sync for BrightnessOverride {}
#[repr(transparent)]
pub struct BrightnessOverrideSettings(::windows_core::IUnknown);
impl BrightnessOverrideSettings {
    pub fn DesiredLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredLevel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DesiredNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn CreateFromLevel(level: f64) -> ::windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromLevel)(::windows_core::Interface::as_raw(this), level, result__.as_mut_ptr()).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    pub fn CreateFromNits(nits: f32) -> ::windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNits)(::windows_core::Interface::as_raw(this), nits, result__.as_mut_ptr()).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    pub fn CreateFromDisplayBrightnessOverrideScenario(overridescenario: DisplayBrightnessOverrideScenario) -> ::windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDisplayBrightnessOverrideScenario)(::windows_core::Interface::as_raw(this), overridescenario, result__.as_mut_ptr()).from_abi::<BrightnessOverrideSettings>(result__)
        })
    }
    pub fn IBrightnessOverrideSettingsStatics<R, F: FnOnce(&IBrightnessOverrideSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BrightnessOverrideSettings, IBrightnessOverrideSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BrightnessOverrideSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BrightnessOverrideSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BrightnessOverrideSettings {}
impl ::core::fmt::Debug for BrightnessOverrideSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrightnessOverrideSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BrightnessOverrideSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.BrightnessOverrideSettings;{d112ab2a-7604-4dba-bcf8-4b6f49502cb0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_Vtbl;
    const IID: ::windows_core::GUID = <IBrightnessOverrideSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverrideSettings";
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows_core::IUnknown {
    fn from(value: BrightnessOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows_core::IUnknown {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BrightnessOverrideSettings> for ::windows_core::IInspectable {
    fn from(value: BrightnessOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BrightnessOverrideSettings> for ::windows_core::IInspectable {
    fn from(value: &BrightnessOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BrightnessOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BrightnessOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BrightnessOverrideSettings {}
unsafe impl ::core::marker::Sync for BrightnessOverrideSettings {}
#[repr(transparent)]
pub struct ColorOverrideSettings(::windows_core::IUnknown);
impl ColorOverrideSettings {
    pub fn DesiredDisplayColorOverrideScenario(&self) -> ::windows_core::Result<DisplayColorOverrideScenario> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayColorOverrideScenario>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDisplayColorOverrideScenario)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayColorOverrideScenario>(result__)
        }
    }
    pub fn CreateFromDisplayColorOverrideScenario(overridescenario: DisplayColorOverrideScenario) -> ::windows_core::Result<ColorOverrideSettings> {
        Self::IColorOverrideSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDisplayColorOverrideScenario)(::windows_core::Interface::as_raw(this), overridescenario, result__.as_mut_ptr()).from_abi::<ColorOverrideSettings>(result__)
        })
    }
    pub fn IColorOverrideSettingsStatics<R, F: FnOnce(&IColorOverrideSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorOverrideSettings, IColorOverrideSettingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorOverrideSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorOverrideSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorOverrideSettings {}
impl ::core::fmt::Debug for ColorOverrideSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorOverrideSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorOverrideSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.ColorOverrideSettings;{fbefa134-4a81-4c4d-a5b6-7d1b5c4bd00b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorOverrideSettings {
    type Vtable = IColorOverrideSettings_Vtbl;
    const IID: ::windows_core::GUID = <IColorOverrideSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.ColorOverrideSettings";
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows_core::IUnknown {
    fn from(value: ColorOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows_core::IUnknown {
    fn from(value: &ColorOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorOverrideSettings> for ::windows_core::IInspectable {
    fn from(value: ColorOverrideSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorOverrideSettings> for ::windows_core::IInspectable {
    fn from(value: &ColorOverrideSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorOverrideSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ColorOverrideSettings {}
unsafe impl ::core::marker::Sync for ColorOverrideSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideOptions {}
impl ::core::clone::Clone for DisplayBrightnessOverrideOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessOverrideOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayBrightnessOverrideOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessOverrideOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayBrightnessOverrideOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayBrightnessOverrideOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayBrightnessOverrideOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideScenario {}
impl ::core::clone::Clone for DisplayBrightnessOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessOverrideScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayBrightnessOverrideScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessOverrideScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayBrightnessOverrideScenario {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayBrightnessScenario {}
impl ::core::clone::Clone for DisplayBrightnessScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayBrightnessScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayBrightnessScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayBrightnessScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayBrightnessScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayBrightnessScenario {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayColorOverrideScenario {}
impl ::core::clone::Clone for DisplayColorOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayColorOverrideScenario {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayColorOverrideScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayColorOverrideScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayColorOverrideScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayColorOverrideScenario {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayColorOverrideScenario;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DisplayEnhancementOverride(::windows_core::IUnknown);
impl DisplayEnhancementOverride {
    pub fn ColorOverrideSettings(&self) -> ::windows_core::Result<ColorOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorOverrideSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ColorOverrideSettings>(result__)
        }
    }
    pub fn SetColorOverrideSettings<'a, Param0: ::windows_core::IntoParam<'a, ColorOverrideSettings>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorOverrideSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn BrightnessOverrideSettings(&self) -> ::windows_core::Result<BrightnessOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BrightnessOverrideSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BrightnessOverrideSettings>(result__)
        }
    }
    pub fn SetBrightnessOverrideSettings<'a, Param0: ::windows_core::IntoParam<'a, BrightnessOverrideSettings>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBrightnessOverrideSettings)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CanOverride(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanOverride)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsOverrideActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverrideActive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> ::windows_core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentDisplayEnhancementOverrideCapabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
    pub fn RequestOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopOverride(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopOverride)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CanOverrideChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).CanOverrideChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanOverrideChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanOverrideChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn IsOverrideActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).IsOverrideActiveChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsOverrideActiveChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayEnhancementOverrideCapabilitiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDisplayEnhancementOverrideCapabilitiesChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisplayEnhancementOverrideCapabilitiesChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<DisplayEnhancementOverride> {
        Self::IDisplayEnhancementOverrideStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayEnhancementOverride>(result__)
        })
    }
    pub fn IDisplayEnhancementOverrideStatics<R, F: FnOnce(&IDisplayEnhancementOverrideStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DisplayEnhancementOverride, IDisplayEnhancementOverrideStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverride {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverride {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverride {}
impl ::core::fmt::Debug for DisplayEnhancementOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayEnhancementOverride {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverride;{429594cf-d97a-4b02-a428-5c4292f7f522})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayEnhancementOverride as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverride";
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows_core::IUnknown {
    fn from(value: DisplayEnhancementOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows_core::IUnknown {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverride> for ::windows_core::IInspectable {
    fn from(value: DisplayEnhancementOverride) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverride> for ::windows_core::IInspectable {
    fn from(value: &DisplayEnhancementOverride) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayEnhancementOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayEnhancementOverride {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverride {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverride {}
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilities(::windows_core::IUnknown);
impl DisplayEnhancementOverrideCapabilities {
    pub fn IsBrightnessControlSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBrightnessControlSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsBrightnessNitsControlSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsBrightnessNitsControlSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedNitRanges(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<NitRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedNitRanges)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<NitRange>>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverrideCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverrideCapabilities {}
impl ::core::fmt::Debug for DisplayEnhancementOverrideCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverrideCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayEnhancementOverrideCapabilities {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities;{457060de-ee5a-47b7-9918-1e51e812ccc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayEnhancementOverrideCapabilities as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows_core::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows_core::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilities> for ::windows_core::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilities> for ::windows_core::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayEnhancementOverrideCapabilities {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilities {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilities {}
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(::windows_core::IUnknown);
impl DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    pub fn Capabilities(&self) -> ::windows_core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Capabilities)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayEnhancementOverrideCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
impl ::core::fmt::Debug for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayEnhancementOverrideCapabilitiesChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs;{db61e664-15fa-49da-8b77-07dbd2af585d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayEnhancementOverrideCapabilitiesChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayEnhancementOverrideCapabilitiesChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DisplayEnhancementOverrideCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
#[repr(transparent)]
pub struct DisplayInformation(::windows_core::IUnknown);
impl DisplayInformation {
    pub fn CurrentOrientation(&self) -> ::windows_core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        }
    }
    pub fn NativeOrientation(&self) -> ::windows_core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).NativeOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        }
    }
    pub fn OrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OrientationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOrientationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ResolutionScale(&self) -> ::windows_core::Result<ResolutionScale> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ResolutionScale>::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResolutionScale>(result__)
        }
    }
    pub fn LogicalDpi(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).LogicalDpi)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RawDpiX(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RawDpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RawDpiY(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RawDpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn DpiChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DpiChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDpiChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDpiChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn StereoEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StereoEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn StereoEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StereoEnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStereoEnabledChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetColorProfileAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetColorProfileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    pub fn ColorProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveColorProfileChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn RawPixelsPerViewPixel(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).RawPixelsPerViewPixel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DiagonalSizeInInches(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DiagonalSizeInInches)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn ScreenWidthInRawPixels(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenWidthInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ScreenHeightInRawPixels(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ScreenHeightInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn GetAdvancedColorInfo(&self) -> ::windows_core::Result<AdvancedColorInfo> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAdvancedColorInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdvancedColorInfo>(result__)
        }
    }
    pub fn AdvancedColorInfoChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AdvancedColorInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdvancedColorInfoChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdvancedColorInfoChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayInformation>(result__)
        })
    }
    pub fn AutoRotationPreferences() -> ::windows_core::Result<DisplayOrientations> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRotationPreferences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        })
    }
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows_core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAutoRotationPreferences)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<DisplayInformation, ::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayContentsInvalidated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IDisplayInformationStatics<R, F: FnOnce(&IDisplayInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DisplayInformation, IDisplayInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayInformation {}
impl ::core::fmt::Debug for DisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayInformation;{bed112ae-adc3-4dc9-ae65-851f4d7d4799})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayInformation";
}
impl ::core::convert::From<DisplayInformation> for ::windows_core::IUnknown {
    fn from(value: DisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows_core::IUnknown {
    fn from(value: &DisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayInformation> for ::windows_core::IInspectable {
    fn from(value: DisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayInformation> for ::windows_core::IInspectable {
    fn from(value: &DisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayInformation {}
unsafe impl ::core::marker::Sync for DisplayInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
impl ::core::marker::Copy for DisplayOrientations {}
impl ::core::clone::Clone for DisplayOrientations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayOrientations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayOrientations {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayOrientations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayOrientations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DisplayOrientations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DisplayOrientations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DisplayOrientations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DisplayOrientations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DisplayOrientations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayOrientations {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayOrientations;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "deprecated")]
pub struct DisplayProperties;
#[cfg(feature = "deprecated")]
impl DisplayProperties {
    #[cfg(feature = "deprecated")]
    pub fn CurrentOrientation() -> ::windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn NativeOrientation() -> ::windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).NativeOrientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn AutoRotationPreferences() -> ::windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayOrientations>::zeroed();
            (::windows_core::Interface::vtable(this).AutoRotationPreferences)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayOrientations>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAutoRotationPreferences)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn OrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).OrientationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveOrientationChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveOrientationChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ResolutionScale() -> ::windows_core::Result<ResolutionScale> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ResolutionScale>::zeroed();
            (::windows_core::Interface::vtable(this).ResolutionScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ResolutionScale>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn LogicalDpi() -> ::windows_core::Result<f32> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).LogicalDpi)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn LogicalDpiChanged<'a, Param0: ::windows_core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LogicalDpiChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveLogicalDpiChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLogicalDpiChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn StereoEnabled() -> ::windows_core::Result<bool> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).StereoEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn StereoEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StereoEnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveStereoEnabledChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveStereoEnabledChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn GetColorProfileAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetColorProfileAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_storage::Streams::IRandomAccessStream>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ColorProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveColorProfileChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveColorProfileChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayContentsInvalidated<'a, Param0: ::windows_core::IntoParam<'a, DisplayPropertiesEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayContentsInvalidated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveDisplayContentsInvalidated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn IDisplayPropertiesStatics<R, F: FnOnce(&IDisplayPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DisplayProperties, IDisplayPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for DisplayProperties {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayProperties";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl DisplayPropertiesEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DisplayPropertiesEventHandlerBox::<F> { vtable: &DisplayPropertiesEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct DisplayPropertiesEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DisplayPropertiesEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DisplayPropertiesEventHandlerBox<F> {
    const VTABLE: DisplayPropertiesEventHandler_Vtbl = DisplayPropertiesEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DisplayPropertiesEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for DisplayPropertiesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for DisplayPropertiesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for DisplayPropertiesEventHandler {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for DisplayPropertiesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayPropertiesEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for DisplayPropertiesEventHandler {
    type Vtable = DisplayPropertiesEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbdd8b01_f1a1_46d1_9ee3_543bcc995980);
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for DisplayPropertiesEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{dbdd8b01-f1a1-46d1-9ee3-543bcc995980}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct DisplayPropertiesEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct DisplayServices(::windows_core::IUnknown);
impl DisplayServices {
    pub fn FindAll() -> ::windows_core::Result<::windows_core::Array<super::DisplayId>> {
        Self::IDisplayServicesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<super::DisplayId>>::zeroed();
            (::windows_core::Interface::vtable(this).FindAll)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<super::DisplayId>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        })
    }
    pub fn IDisplayServicesStatics<R, F: FnOnce(&IDisplayServicesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DisplayServices, IDisplayServicesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayServices {}
impl ::core::fmt::Debug for DisplayServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayServices {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.DisplayServices;{1b54f32b-890d-5747-bd26-fdbdeb0c8a71})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayServices {
    type Vtable = IDisplayServices_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayServices as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayServices";
}
impl ::core::convert::From<DisplayServices> for ::windows_core::IUnknown {
    fn from(value: DisplayServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows_core::IUnknown {
    fn from(value: &DisplayServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayServices> for ::windows_core::IInspectable {
    fn from(value: DisplayServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayServices> for ::windows_core::IInspectable {
    fn from(value: &DisplayServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayServices {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayServices {}
unsafe impl ::core::marker::Sync for DisplayServices {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl ::core::marker::Copy for HdrMetadataFormat {}
impl ::core::clone::Clone for HdrMetadataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdrMetadataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HdrMetadataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdrMetadataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdrMetadataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HdrMetadataFormat {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.HdrMetadataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedColorInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8797dcfb_b229_4081_ae9a_2cc85e34ad6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedColorInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdvancedColorKind) -> ::windows_core::HRESULT,
    pub RedPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub GreenPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub BluePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub WhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub MaxLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SdrWhiteLevelInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: HdrMetadataFormat, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: AdvancedColorKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrightnessOverride {
    type Vtable = IBrightnessOverride_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96c9621a_c143_4392_bedd_4a7e9574c8fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverride_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows_core::HRESULT,
    pub SetBrightnessScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows_core::HRESULT,
    pub GetLevelForScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scenario: DisplayBrightnessScenario, result__: *mut f64) -> ::windows_core::HRESULT,
    pub StartOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub BrightnessLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBrightnessLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd112ab2a_7604_4dba_bcf8_4b6f49502cb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DesiredNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrightnessOverrideSettingsStatics {
    type Vtable = IBrightnessOverrideSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd487dc90_6f74_440b_b383_5fe96cf00b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nits: f32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateFromDisplayBrightnessOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overridescenario: DisplayBrightnessOverrideScenario, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBrightnessOverrideStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrightnessOverrideStatics {
    type Vtable = IBrightnessOverrideStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03a7b9ed_e1f1_4a68_a11f_946ad8ce5393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrightnessOverrideStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefaultForSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SaveForSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorOverrideSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorOverrideSettings {
    type Vtable = IColorOverrideSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbefa134_4a81_4c4d_a5b6_7d1b5c4bd00b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesiredDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayColorOverrideScenario) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorOverrideSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorOverrideSettingsStatics {
    type Vtable = IColorOverrideSettingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb068e05f_c41f_4ac9_afab_827ab6248f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorOverrideSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateFromDisplayColorOverrideScenario: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overridescenario: DisplayColorOverrideScenario, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x429594cf_d97a_4b02_a428_5c4292f7f522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverride_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetColorOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BrightnessOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBrightnessOverrideSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetCurrentDisplayEnhancementOverrideCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CanOverrideChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanOverrideChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x457060de_ee5a_47b7_9918_1e51e812ccc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsBrightnessControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBrightnessNitsControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedNitRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedNitRanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb61e664_15fa_49da_8b77_07dbd2af585d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayEnhancementOverrideStatics {
    type Vtable = IDisplayEnhancementOverrideStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf5b7ec1_9791_4453_b013_29b6f778e519);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayEnhancementOverrideStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbed112ae_adc3_4dc9_ae65_851f4d7d4799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    pub NativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ResolutionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows_core::HRESULT,
    pub LogicalDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub DpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetColorProfileAsync: usize,
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformation2 {
    type Vtable = IDisplayInformation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4dcd0021_fad1_4b8e_8edf_775887b8bf19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RawPixelsPerViewPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformation3 {
    type Vtable = IDisplayInformation3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb15011d_0f09_4466_8ff3_11de9a3c929a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DiagonalSizeInInches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformation4 {
    type Vtable = IDisplayInformation4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc972ce2f_1242_46be_b536_e1aafe9e7acf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScreenWidthInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ScreenHeightInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformation5 {
    type Vtable = IDisplayInformation5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a5442dc_2cde_4a8d_80d1_21dc5adcc1aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAdvancedColorInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayInformationStatics {
    type Vtable = IDisplayInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6a02a6c_d452_44dc_ba07_96f3c6adf9d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows_core::HRESULT,
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDisplayPropertiesStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IDisplayPropertiesStatics {
    type Vtable = IDisplayPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6937ed8d_30ea_4ded_8271_4553ff02f68a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub CurrentOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub NativeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NativeOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub AutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub SetAutoRotationPreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DisplayOrientations) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub OrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveOrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ResolutionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResolutionScale) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResolutionScale: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpi: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveLogicalDpiChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveLogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabledChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    GetColorProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ColorProfileChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveColorProfileChanged: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayContentsInvalidated: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveDisplayContentsInvalidated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayServices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayServices {
    type Vtable = IDisplayServices_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b54f32b_890d_5747_bd26_fdbdeb0c8a71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServices_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayServicesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayServicesStatics {
    type Vtable = IDisplayServicesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc2096bf_730a_5560_b461_91c13d692e0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayServicesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FindAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::DisplayId) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl ::core::marker::Copy for NitRange {}
impl ::core::clone::Clone for NitRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NitRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NitRange").field("MinNits", &self.MinNits).field("MaxNits", &self.MaxNits).field("StepSizeNits", &self.StepSizeNits).finish()
    }
}
unsafe impl ::windows_core::Abi for NitRange {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for NitRange {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.NitRange;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for NitRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NitRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for NitRange {}
impl ::core::default::Default for NitRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: Self = Self(0i32);
    pub const Scale100Percent: Self = Self(100i32);
    pub const Scale120Percent: Self = Self(120i32);
    pub const Scale125Percent: Self = Self(125i32);
    pub const Scale140Percent: Self = Self(140i32);
    pub const Scale150Percent: Self = Self(150i32);
    pub const Scale160Percent: Self = Self(160i32);
    pub const Scale175Percent: Self = Self(175i32);
    pub const Scale180Percent: Self = Self(180i32);
    pub const Scale200Percent: Self = Self(200i32);
    pub const Scale225Percent: Self = Self(225i32);
    pub const Scale250Percent: Self = Self(250i32);
    pub const Scale300Percent: Self = Self(300i32);
    pub const Scale350Percent: Self = Self(350i32);
    pub const Scale400Percent: Self = Self(400i32);
    pub const Scale450Percent: Self = Self(450i32);
    pub const Scale500Percent: Self = Self(500i32);
}
impl ::core::marker::Copy for ResolutionScale {}
impl ::core::clone::Clone for ResolutionScale {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResolutionScale {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ResolutionScale {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResolutionScale {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResolutionScale").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ResolutionScale {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.ResolutionScale;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
